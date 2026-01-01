/* FP:vec_cache.rs-0001 */ // VecCache maintains a mapping from K -> (V, I) pairing. K and I must be roughly u32-sized, and V
/* FP:vec_cache.rs-0002 */ // must be Copy.
/* FP:vec_cache.rs-0003 */ //
/* FP:vec_cache.rs-0004 */ // VecCache supports efficient concurrent put/get across the key space, with write-once semantics
/* FP:vec_cache.rs-0005 */ // (i.e., a given key can only be put once). Subsequent puts will panic.
/* FP:vec_cache.rs-0006 */ //
/* FP:vec_cache.rs-0007 */ // This is currently used for query caching.
/* FP:vec_cache.rs-0008 */ 
/* FP:vec_cache.rs-0009 */ use std::fmt::Debug;
/* FP:vec_cache.rs-0010 */ use std::marker::PhantomData;
/* FP:vec_cache.rs-0011 */ use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicUsize, Ordering};
/* FP:vec_cache.rs-0012 */ 
/* FP:vec_cache.rs-0013 */ use crate::rustc_index::Idx;
/* FP:vec_cache.rs-0014 */ 
/* FP:vec_cache.rs-0015 */ struct Slot<V> {
/* FP:vec_cache.rs-0016 */     // We never construct &Slot<V> so it's fine for this to not be in an UnsafeCell.
/* FP:vec_cache.rs-0017 */     value: V,
/* FP:vec_cache.rs-0018 */     // This is both an index and a once-lock.
/* FP:vec_cache.rs-0019 */     //
/* FP:vec_cache.rs-0020 */     // 0: not yet initialized.
/* FP:vec_cache.rs-0021 */     // 1: lock held, initializing.
/* FP:vec_cache.rs-0022 */     // 2..u32::MAX - 2: initialized.
/* FP:vec_cache.rs-0023 */     index_and_lock: AtomicU32,
/* FP:vec_cache.rs-0024 */ }
/* FP:vec_cache.rs-0025 */ 
/* FP:vec_cache.rs-0026 */ /// This uniquely identifies a single `Slot<V>` entry in the buckets map, and provides accessors for
/* FP:vec_cache.rs-0027 */ /// either getting the value or putting a value.
/* FP:vec_cache.rs-0028 */ #[derive(Copy, Clone, Debug)]
/* FP:vec_cache.rs-0029 */ struct SlotIndex {
/* FP:vec_cache.rs-0030 */     // the index of the bucket in VecCache (0 to 20)
/* FP:vec_cache.rs-0031 */     bucket_idx: usize,
/* FP:vec_cache.rs-0032 */     // number of entries in that bucket
/* FP:vec_cache.rs-0033 */     entries: usize,
/* FP:vec_cache.rs-0034 */     // the index of the slot within the bucket
/* FP:vec_cache.rs-0035 */     index_in_bucket: usize,
/* FP:vec_cache.rs-0036 */ }
/* FP:vec_cache.rs-0037 */ 
/* FP:vec_cache.rs-0038 */ // This makes sure the counts are consistent with what we allocate, precomputing each bucket a
/* FP:vec_cache.rs-0039 */ // compile-time. Visiting all powers of two is enough to hit all the buckets.
/* FP:vec_cache.rs-0040 */ //
/* FP:vec_cache.rs-0041 */ // We confirm counts are accurate in the slot_index_exhaustive test.
/* FP:vec_cache.rs-0042 */ const ENTRIES_BY_BUCKET: [usize; 21] = {
/* FP:vec_cache.rs-0043 */     let mut entries = [0; 21];
/* FP:vec_cache.rs-0044 */     let mut key = 0;
/* FP:vec_cache.rs-0045 */     loop {
/* FP:vec_cache.rs-0046 */         let si = SlotIndex::from_index(key);
/* FP:vec_cache.rs-0047 */         entries[si.bucket_idx] = si.entries;
/* FP:vec_cache.rs-0048 */         if key == 0 {
/* FP:vec_cache.rs-0049 */             key = 1;
/* FP:vec_cache.rs-0050 */         } else if key == (1 << 31) {
/* FP:vec_cache.rs-0051 */             break;
/* FP:vec_cache.rs-0052 */         } else {
/* FP:vec_cache.rs-0053 */             key <<= 1;
/* FP:vec_cache.rs-0054 */         }
/* FP:vec_cache.rs-0055 */     }
/* FP:vec_cache.rs-0056 */     entries
/* FP:vec_cache.rs-0057 */ };
/* FP:vec_cache.rs-0058 */ 
/* FP:vec_cache.rs-0059 */ impl SlotIndex {
/* FP:vec_cache.rs-0060 */     // This unpacks a flat u32 index into identifying which bucket it belongs to and the offset
/* FP:vec_cache.rs-0061 */     // within that bucket. As noted in the VecCache docs, buckets double in size with each index.
/* FP:vec_cache.rs-0062 */     // Typically that would mean 31 buckets (2^0 + 2^1 ... + 2^31 = u32::MAX - 1), but to reduce
/* FP:vec_cache.rs-0063 */     // the size of the VecCache struct and avoid uselessly small allocations, we instead have the
/* FP:vec_cache.rs-0064 */     // first bucket have 2**12 entries. To simplify the math, the second bucket also 2**12 entries,
/* FP:vec_cache.rs-0065 */     // and buckets double from there.
/* FP:vec_cache.rs-0066 */     //
/* FP:vec_cache.rs-0067 */     // We assert that [0, 2**32 - 1] uniquely map through this function to individual, consecutive
/* FP:vec_cache.rs-0068 */     // slots (see `slot_index_exhaustive` in tests).
/* FP:vec_cache.rs-0069 */     #[inline]
/* FP:vec_cache.rs-0070 */     const fn from_index(idx: u32) -> Self {
/* FP:vec_cache.rs-0071 */         const FIRST_BUCKET_SHIFT: usize = 12;
/* FP:vec_cache.rs-0072 */         if idx < (1 << FIRST_BUCKET_SHIFT) {
/* FP:vec_cache.rs-0073 */             return SlotIndex {
/* FP:vec_cache.rs-0074 */                 bucket_idx: 0,
/* FP:vec_cache.rs-0075 */                 entries: 1 << FIRST_BUCKET_SHIFT,
/* FP:vec_cache.rs-0076 */                 index_in_bucket: idx as usize,
/* FP:vec_cache.rs-0077 */             };
/* FP:vec_cache.rs-0078 */         }
/* FP:vec_cache.rs-0079 */         // We already ruled out idx 0, so this `ilog2` never panics (and the check optimizes away)
/* FP:vec_cache.rs-0080 */         let bucket = idx.ilog2() as usize;
/* FP:vec_cache.rs-0081 */         let entries = 1 << bucket;
/* FP:vec_cache.rs-0082 */         SlotIndex {
/* FP:vec_cache.rs-0083 */             bucket_idx: bucket - FIRST_BUCKET_SHIFT + 1,
/* FP:vec_cache.rs-0084 */             entries,
/* FP:vec_cache.rs-0085 */             index_in_bucket: idx as usize - entries,
/* FP:vec_cache.rs-0086 */         }
/* FP:vec_cache.rs-0087 */     }
/* FP:vec_cache.rs-0088 */ 
/* FP:vec_cache.rs-0089 */     // SAFETY: Buckets must be managed solely by functions here (i.e., get/put on SlotIndex) and
/* FP:vec_cache.rs-0090 */     // `self` comes from SlotIndex::from_index
/* FP:vec_cache.rs-0091 */     #[inline]
/* FP:vec_cache.rs-0092 */     unsafe fn get<V: Copy>(&self, buckets: &[AtomicPtr<Slot<V>>; 21]) -> Option<(V, u32)> {
/* FP:vec_cache.rs-0093 */         // SAFETY: `bucket_idx` is ilog2(u32).saturating_sub(11), which is at most 21, i.e.,
/* FP:vec_cache.rs-0094 */         // in-bounds of buckets. See `from_index` for computation.
/* FP:vec_cache.rs-0095 */         let bucket = unsafe { buckets.get_unchecked(self.bucket_idx) };
/* FP:vec_cache.rs-0096 */         let ptr = bucket.load(Ordering::Acquire);
/* FP:vec_cache.rs-0097 */         // Bucket is not yet initialized: then we obviously won't find this entry in that bucket.
/* FP:vec_cache.rs-0098 */         if ptr.is_null() {
/* FP:vec_cache.rs-0099 */             return None;
/* FP:vec_cache.rs-0100 */         }
/* FP:vec_cache.rs-0101 */         assert!(self.index_in_bucket < self.entries);
/* FP:vec_cache.rs-0102 */         // SAFETY: `bucket` was allocated (so <= isize in total bytes) to hold `entries`, so this
/* FP:vec_cache.rs-0103 */         // must be inbounds.
/* FP:vec_cache.rs-0104 */         let slot = unsafe { ptr.add(self.index_in_bucket) };
/* FP:vec_cache.rs-0105 */ 
/* FP:vec_cache.rs-0106 */         // SAFETY: initialized bucket has zeroed all memory within the bucket, so we are valid for
/* FP:vec_cache.rs-0107 */         // AtomicU32 access.
/* FP:vec_cache.rs-0108 */         let index_and_lock = unsafe { &(*slot).index_and_lock };
/* FP:vec_cache.rs-0109 */         let current = index_and_lock.load(Ordering::Acquire);
/* FP:vec_cache.rs-0110 */         let index = match current {
/* FP:vec_cache.rs-0111 */             0 => return None,
/* FP:vec_cache.rs-0112 */             // Treat "initializing" as actually just not initialized at all.
/* FP:vec_cache.rs-0113 */             // The only reason this is a separate state is that `complete` calls could race and
/* FP:vec_cache.rs-0114 */             // we can't allow that, but from load perspective there's no difference.
/* FP:vec_cache.rs-0115 */             1 => return None,
/* FP:vec_cache.rs-0116 */             _ => current - 2,
/* FP:vec_cache.rs-0117 */         };
/* FP:vec_cache.rs-0118 */ 
/* FP:vec_cache.rs-0119 */         // SAFETY:
/* FP:vec_cache.rs-0120 */         // * slot is a valid pointer (buckets are always valid for the index we get).
/* FP:vec_cache.rs-0121 */         // * value is initialized since we saw a >= 2 index above.
/* FP:vec_cache.rs-0122 */         // * `V: Copy`, so safe to read.
/* FP:vec_cache.rs-0123 */         let value = unsafe { (*slot).value };
/* FP:vec_cache.rs-0124 */         Some((value, index))
/* FP:vec_cache.rs-0125 */     }
/* FP:vec_cache.rs-0126 */ 
/* FP:vec_cache.rs-0127 */     fn bucket_ptr<V>(&self, bucket: &AtomicPtr<Slot<V>>) -> *mut Slot<V> {
/* FP:vec_cache.rs-0128 */         let ptr = bucket.load(Ordering::Acquire);
/* FP:vec_cache.rs-0129 */         if ptr.is_null() { self.initialize_bucket(bucket) } else { ptr }
/* FP:vec_cache.rs-0130 */     }
/* FP:vec_cache.rs-0131 */ 
/* FP:vec_cache.rs-0132 */     #[cold]
/* FP:vec_cache.rs-0133 */     fn initialize_bucket<V>(&self, bucket: &AtomicPtr<Slot<V>>) -> *mut Slot<V> {
/* FP:vec_cache.rs-0134 */         static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
/* FP:vec_cache.rs-0135 */ 
/* FP:vec_cache.rs-0136 */         // If we are initializing the bucket, then acquire a global lock.
/* FP:vec_cache.rs-0137 */         //
/* FP:vec_cache.rs-0138 */         // This path is quite cold, so it's cheap to use a global lock. This ensures that we never
/* FP:vec_cache.rs-0139 */         // have multiple allocations for the same bucket.
/* FP:vec_cache.rs-0140 */         let _allocator_guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
/* FP:vec_cache.rs-0141 */ 
/* FP:vec_cache.rs-0142 */         let ptr = bucket.load(Ordering::Acquire);
/* FP:vec_cache.rs-0143 */ 
/* FP:vec_cache.rs-0144 */         // OK, now under the allocator lock, if we're still null then it's definitely us that will
/* FP:vec_cache.rs-0145 */         // initialize this bucket.
/* FP:vec_cache.rs-0146 */         if ptr.is_null() {
/* FP:vec_cache.rs-0147 */             let bucket_layout =
/* FP:vec_cache.rs-0148 */                 std::alloc::Layout::array::<Slot<V>>(self.entries as usize).unwrap();
/* FP:vec_cache.rs-0149 */             // This is more of a sanity check -- this code is very cold, so it's safe to pay a
/* FP:vec_cache.rs-0150 */             // little extra cost here.
/* FP:vec_cache.rs-0151 */             assert!(bucket_layout.size() > 0);
/* FP:vec_cache.rs-0152 */             // SAFETY: Just checked that size is non-zero.
/* FP:vec_cache.rs-0153 */             let allocated = unsafe { std::alloc::alloc_zeroed(bucket_layout).cast::<Slot<V>>() };
/* FP:vec_cache.rs-0154 */             if allocated.is_null() {
/* FP:vec_cache.rs-0155 */                 std::alloc::handle_alloc_error(bucket_layout);
/* FP:vec_cache.rs-0156 */             }
/* FP:vec_cache.rs-0157 */             bucket.store(allocated, Ordering::Release);
/* FP:vec_cache.rs-0158 */             allocated
/* FP:vec_cache.rs-0159 */         } else {
/* FP:vec_cache.rs-0160 */             // Otherwise some other thread initialized this bucket after we took the lock. In that
/* FP:vec_cache.rs-0161 */             // case, just return early.
/* FP:vec_cache.rs-0162 */             ptr
/* FP:vec_cache.rs-0163 */         }
/* FP:vec_cache.rs-0164 */     }
/* FP:vec_cache.rs-0165 */ 
/* FP:vec_cache.rs-0166 */     /// Returns true if this successfully put into the map.
/* FP:vec_cache.rs-0167 */     #[inline]
/* FP:vec_cache.rs-0168 */     fn put<V>(&self, buckets: &[AtomicPtr<Slot<V>>; 21], value: V, extra: u32) -> bool {
/* FP:vec_cache.rs-0169 */         // SAFETY: `bucket_idx` is ilog2(u32).saturating_sub(11), which is at most 21, i.e.,
/* FP:vec_cache.rs-0170 */         // in-bounds of buckets.
/* FP:vec_cache.rs-0171 */         let bucket = unsafe { buckets.get_unchecked(self.bucket_idx) };
/* FP:vec_cache.rs-0172 */         let ptr = self.bucket_ptr(bucket);
/* FP:vec_cache.rs-0173 */ 
/* FP:vec_cache.rs-0174 */         assert!(self.index_in_bucket < self.entries);
/* FP:vec_cache.rs-0175 */         // SAFETY: `bucket` was allocated (so <= isize in total bytes) to hold `entries`, so this
/* FP:vec_cache.rs-0176 */         // must be inbounds.
/* FP:vec_cache.rs-0177 */         let slot = unsafe { ptr.add(self.index_in_bucket) };
/* FP:vec_cache.rs-0178 */ 
/* FP:vec_cache.rs-0179 */         // SAFETY: initialized bucket has zeroed all memory within the bucket, so we are valid for
/* FP:vec_cache.rs-0180 */         // AtomicU32 access.
/* FP:vec_cache.rs-0181 */         let index_and_lock = unsafe { &(*slot).index_and_lock };
/* FP:vec_cache.rs-0182 */         match index_and_lock.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {
/* FP:vec_cache.rs-0183 */             Ok(_) => {
/* FP:vec_cache.rs-0184 */                 // We have acquired the initialization lock. It is our job to write `value` and
/* FP:vec_cache.rs-0185 */                 // then set the lock to the real index.
/* FP:vec_cache.rs-0186 */ 
/* FP:vec_cache.rs-0187 */                 unsafe {
/* FP:vec_cache.rs-0188 */                     (&raw mut (*slot).value).write(value);
/* FP:vec_cache.rs-0189 */                 }
/* FP:vec_cache.rs-0190 */ 
/* FP:vec_cache.rs-0191 */                 index_and_lock.store(extra.checked_add(2).unwrap(), Ordering::Release);
/* FP:vec_cache.rs-0192 */ 
/* FP:vec_cache.rs-0193 */                 true
/* FP:vec_cache.rs-0194 */             }
/* FP:vec_cache.rs-0195 */ 
/* FP:vec_cache.rs-0196 */             // Treat "initializing" as the caller's fault. Callers are responsible for ensuring that
/* FP:vec_cache.rs-0197 */             // there are no races on initialization. In the compiler's current usage for query
/* FP:vec_cache.rs-0198 */             // caches, that's the "active query map" which ensures each query actually runs once
/* FP:vec_cache.rs-0199 */             // (even if concurrently started).
/* FP:vec_cache.rs-0200 */             Err(1) => panic!("caller raced calls to put()"),
/* FP:vec_cache.rs-0201 */ 
/* FP:vec_cache.rs-0202 */             // This slot was already populated. Also ignore, currently this is the same as
/* FP:vec_cache.rs-0203 */             // "initializing".
/* FP:vec_cache.rs-0204 */             Err(_) => false,
/* FP:vec_cache.rs-0205 */         }
/* FP:vec_cache.rs-0206 */     }
/* FP:vec_cache.rs-0207 */ }
/* FP:vec_cache.rs-0208 */ 
/* FP:vec_cache.rs-0209 */ /// In-memory cache for queries whose keys are densely-numbered IDs
/* FP:vec_cache.rs-0210 */ /// (e.g `CrateNum`, `LocalDefId`), and can therefore be used as indices
/* FP:vec_cache.rs-0211 */ /// into a dense vector of cached values.
/* FP:vec_cache.rs-0212 */ ///
/* FP:vec_cache.rs-0213 */ /// (As of [#124780] the underlying storage is not an actual `Vec`, but rather
/* FP:vec_cache.rs-0214 */ /// a series of increasingly-large buckets, for improved performance when the
/* FP:vec_cache.rs-0215 */ /// parallel frontend is using multiple threads.)
/* FP:vec_cache.rs-0216 */ ///
/* FP:vec_cache.rs-0217 */ /// Each entry in the cache stores the query's return value (`V`), and also
/* FP:vec_cache.rs-0218 */ /// an associated index (`I`), which in practice is a `DepNodeIndex` used for
/* FP:vec_cache.rs-0219 */ /// query dependency tracking.
/* FP:vec_cache.rs-0220 */ ///
/* FP:vec_cache.rs-0221 */ /// [#124780]: https://github.com/rust-lang/rust/pull/124780
/* FP:vec_cache.rs-0222 */ pub struct VecCache<K: Idx, V, I> {
/* FP:vec_cache.rs-0223 */     // Entries per bucket:
/* FP:vec_cache.rs-0224 */     // Bucket  0:       4096 2^12
/* FP:vec_cache.rs-0225 */     // Bucket  1:       4096 2^12
/* FP:vec_cache.rs-0226 */     // Bucket  2:       8192
/* FP:vec_cache.rs-0227 */     // Bucket  3:      16384
/* FP:vec_cache.rs-0228 */     // ...
/* FP:vec_cache.rs-0229 */     // Bucket 19: 1073741824
/* FP:vec_cache.rs-0230 */     // Bucket 20: 2147483648
/* FP:vec_cache.rs-0231 */     // The total number of entries if all buckets are initialized is u32::MAX-1.
/* FP:vec_cache.rs-0232 */     buckets: [AtomicPtr<Slot<V>>; 21],
/* FP:vec_cache.rs-0233 */ 
/* FP:vec_cache.rs-0234 */     // In the compiler's current usage these are only *read* during incremental and self-profiling.
/* FP:vec_cache.rs-0235 */     // They are an optimization over iterating the full buckets array.
/* FP:vec_cache.rs-0236 */     present: [AtomicPtr<Slot<()>>; 21],
/* FP:vec_cache.rs-0237 */     len: AtomicUsize,
/* FP:vec_cache.rs-0238 */ 
/* FP:vec_cache.rs-0239 */     key: PhantomData<(K, I)>,
/* FP:vec_cache.rs-0240 */ }
/* FP:vec_cache.rs-0241 */ 
/* FP:vec_cache.rs-0242 */ impl<K: Idx, V, I> Default for VecCache<K, V, I> {
/* FP:vec_cache.rs-0243 */     fn default() -> Self {
/* FP:vec_cache.rs-0244 */         VecCache {
/* FP:vec_cache.rs-0245 */             buckets: Default::default(),
/* FP:vec_cache.rs-0246 */             key: PhantomData,
/* FP:vec_cache.rs-0247 */             len: Default::default(),
/* FP:vec_cache.rs-0248 */             present: Default::default(),
/* FP:vec_cache.rs-0249 */         }
/* FP:vec_cache.rs-0250 */     }
/* FP:vec_cache.rs-0251 */ }
/* FP:vec_cache.rs-0252 */ 
/* FP:vec_cache.rs-0253 */ // SAFETY: No access to `V` is made.
/* FP:vec_cache.rs-0254 */ unsafe impl<K: Idx, #[may_dangle] V, I> Drop for VecCache<K, V, I> {
/* FP:vec_cache.rs-0255 */     fn drop(&mut self) {
/* FP:vec_cache.rs-0256 */         // We have unique ownership, so no locks etc. are needed. Since `K` and `V` are both `Copy`,
/* FP:vec_cache.rs-0257 */         // we are also guaranteed to just need to deallocate any large arrays (not iterate over
/* FP:vec_cache.rs-0258 */         // contents).
/* FP:vec_cache.rs-0259 */         //
/* FP:vec_cache.rs-0260 */         // Confirm no need to deallocate individual entries. Note that `V: Copy` is asserted on
/* FP:vec_cache.rs-0261 */         // insert/lookup but not necessarily construction, primarily to avoid annoyingly propagating
/* FP:vec_cache.rs-0262 */         // the bounds into struct definitions everywhere.
/* FP:vec_cache.rs-0263 */         assert!(!std::mem::needs_drop::<K>());
/* FP:vec_cache.rs-0264 */         assert!(!std::mem::needs_drop::<V>());
/* FP:vec_cache.rs-0265 */ 
/* FP:vec_cache.rs-0266 */         for (idx, bucket) in self.buckets.iter().enumerate() {
/* FP:vec_cache.rs-0267 */             let bucket = bucket.load(Ordering::Acquire);
/* FP:vec_cache.rs-0268 */             if !bucket.is_null() {
/* FP:vec_cache.rs-0269 */                 let layout = std::alloc::Layout::array::<Slot<V>>(ENTRIES_BY_BUCKET[idx]).unwrap();
/* FP:vec_cache.rs-0270 */                 unsafe {
/* FP:vec_cache.rs-0271 */                     std::alloc::dealloc(bucket.cast(), layout);
/* FP:vec_cache.rs-0272 */                 }
/* FP:vec_cache.rs-0273 */             }
/* FP:vec_cache.rs-0274 */         }
/* FP:vec_cache.rs-0275 */ 
/* FP:vec_cache.rs-0276 */         for (idx, bucket) in self.present.iter().enumerate() {
/* FP:vec_cache.rs-0277 */             let bucket = bucket.load(Ordering::Acquire);
/* FP:vec_cache.rs-0278 */             if !bucket.is_null() {
/* FP:vec_cache.rs-0279 */                 let layout = std::alloc::Layout::array::<Slot<()>>(ENTRIES_BY_BUCKET[idx]).unwrap();
/* FP:vec_cache.rs-0280 */                 unsafe {
/* FP:vec_cache.rs-0281 */                     std::alloc::dealloc(bucket.cast(), layout);
/* FP:vec_cache.rs-0282 */                 }
/* FP:vec_cache.rs-0283 */             }
/* FP:vec_cache.rs-0284 */         }
/* FP:vec_cache.rs-0285 */     }
/* FP:vec_cache.rs-0286 */ }
/* FP:vec_cache.rs-0287 */ 
/* FP:vec_cache.rs-0288 */ impl<K, V, I> VecCache<K, V, I>
/* FP:vec_cache.rs-0289 */ where
/* FP:vec_cache.rs-0290 */     K: Eq + Idx + Copy + Debug,
/* FP:vec_cache.rs-0291 */     V: Copy,
/* FP:vec_cache.rs-0292 */     I: Idx + Copy,
/* FP:vec_cache.rs-0293 */ {
/* FP:vec_cache.rs-0294 */     #[inline(always)]
/* FP:vec_cache.rs-0295 */     pub fn lookup(&self, key: &K) -> Option<(V, I)> {
/* FP:vec_cache.rs-0296 */         let key = u32::try_from(key.index()).unwrap();
/* FP:vec_cache.rs-0297 */         let slot_idx = SlotIndex::from_index(key);
/* FP:vec_cache.rs-0298 */         match unsafe { slot_idx.get(&self.buckets) } {
/* FP:vec_cache.rs-0299 */             Some((value, idx)) => Some((value, I::new(idx as usize))),
/* FP:vec_cache.rs-0300 */             None => None,
/* FP:vec_cache.rs-0301 */         }
/* FP:vec_cache.rs-0302 */     }
/* FP:vec_cache.rs-0303 */ 
/* FP:vec_cache.rs-0304 */     #[inline]
/* FP:vec_cache.rs-0305 */     pub fn complete(&self, key: K, value: V, index: I) {
/* FP:vec_cache.rs-0306 */         let key = u32::try_from(key.index()).unwrap();
/* FP:vec_cache.rs-0307 */         let slot_idx = SlotIndex::from_index(key);
/* FP:vec_cache.rs-0308 */         if slot_idx.put(&self.buckets, value, index.index() as u32) {
/* FP:vec_cache.rs-0309 */             let present_idx = self.len.fetch_add(1, Ordering::Relaxed);
/* FP:vec_cache.rs-0310 */             let slot = SlotIndex::from_index(present_idx as u32);
/* FP:vec_cache.rs-0311 */             // We should always be uniquely putting due to `len` fetch_add returning unique values.
/* FP:vec_cache.rs-0312 */             assert!(slot.put(&self.present, (), key));
/* FP:vec_cache.rs-0313 */         }
/* FP:vec_cache.rs-0314 */     }
/* FP:vec_cache.rs-0315 */ 
/* FP:vec_cache.rs-0316 */     pub fn iter(&self, f: &mut dyn FnMut(&K, &V, I)) {
/* FP:vec_cache.rs-0317 */         for idx in 0..self.len.load(Ordering::Acquire) {
/* FP:vec_cache.rs-0318 */             let key = SlotIndex::from_index(idx as u32);
/* FP:vec_cache.rs-0319 */             match unsafe { key.get(&self.present) } {
/* FP:vec_cache.rs-0320 */                 // This shouldn't happen in our current usage (iter is really only
/* FP:vec_cache.rs-0321 */                 // used long after queries are done running), but if we hit this in practice it's
/* FP:vec_cache.rs-0322 */                 // probably fine to just break early.
/* FP:vec_cache.rs-0323 */                 None => unreachable!(),
/* FP:vec_cache.rs-0324 */                 Some(((), key)) => {
/* FP:vec_cache.rs-0325 */                     let key = K::new(key as usize);
/* FP:vec_cache.rs-0326 */                     // unwrap() is OK: present entries are always written only after we put the real
/* FP:vec_cache.rs-0327 */                     // entry.
/* FP:vec_cache.rs-0328 */                     let value = self.lookup(&key).unwrap();
/* FP:vec_cache.rs-0329 */                     f(&key, &value.0, value.1);
/* FP:vec_cache.rs-0330 */                 }
/* FP:vec_cache.rs-0331 */             }
/* FP:vec_cache.rs-0332 */         }
/* FP:vec_cache.rs-0333 */     }
/* FP:vec_cache.rs-0334 */ }
/* FP:vec_cache.rs-0335 */ 
/* FP:vec_cache.rs-0336 */ #[cfg(test)]