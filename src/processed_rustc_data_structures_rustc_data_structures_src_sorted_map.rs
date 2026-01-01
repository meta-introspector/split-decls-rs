/* FP:sorted_map.rs-0001 */ use std::borrow::Borrow;
/* FP:sorted_map.rs-0002 */ use std::cmp::Ordering;
/* FP:sorted_map.rs-0003 */ use std::fmt::Debug;
/* FP:sorted_map.rs-0004 */ use std::mem;
/* FP:sorted_map.rs-0005 */ use std::ops::{Bound, Index, IndexMut, RangeBounds};
/* FP:sorted_map.rs-0006 */ 
/* FP:sorted_map.rs-0007 */ use rustc_macros::{Decodable_NoContext, Encodable_NoContext};
/* FP:sorted_map.rs-0008 */ 
/* FP:sorted_map.rs-0009 */ use crate::stable_hasher::{HashStable, StableHasher, StableOrd};
/* FP:sorted_map.rs-0010 */ 
/* FP:sorted_map.rs-0012 */ 
/* FP:sorted_map.rs-0013 */ pub use index_map::SortedIndexMultiMap;
/* FP:sorted_map.rs-0014 */ 
/* FP:sorted_map.rs-0015 */ /// `SortedMap` is a data structure with similar characteristics as BTreeMap but
/* FP:sorted_map.rs-0016 */ /// slightly different trade-offs: lookup is *O*(log(*n*)), insertion and removal
/* FP:sorted_map.rs-0017 */ /// are *O*(*n*) but elements can be iterated in order cheaply.
/* FP:sorted_map.rs-0018 */ ///
/* FP:sorted_map.rs-0019 */ /// `SortedMap` can be faster than a `BTreeMap` for small sizes (<50) since it
/* FP:sorted_map.rs-0020 */ /// stores data in a more compact way. It also supports accessing contiguous
/* FP:sorted_map.rs-0021 */ /// ranges of elements as a slice, and slices of already sorted elements can be
/* FP:sorted_map.rs-0022 */ /// inserted efficiently.
/* FP:sorted_map.rs-0023 */ #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encodable_NoContext, Decodable_NoContext)]
/* FP:sorted_map.rs-0024 */ pub struct SortedMap<K, V> {
/* FP:sorted_map.rs-0025 */     data: Vec<(K, V)>,
/* FP:sorted_map.rs-0026 */ }
/* FP:sorted_map.rs-0027 */ 
/* FP:sorted_map.rs-0028 */ impl<K, V> Default for SortedMap<K, V> {
/* FP:sorted_map.rs-0029 */     #[inline]
/* FP:sorted_map.rs-0030 */     fn default() -> SortedMap<K, V> {
/* FP:sorted_map.rs-0031 */         SortedMap { data: Vec::new() }
/* FP:sorted_map.rs-0032 */     }
/* FP:sorted_map.rs-0033 */ }
/* FP:sorted_map.rs-0034 */ 
/* FP:sorted_map.rs-0035 */ impl<K, V> SortedMap<K, V> {
/* FP:sorted_map.rs-0036 */     #[inline]
/* FP:sorted_map.rs-0037 */     pub const fn new() -> SortedMap<K, V> {
/* FP:sorted_map.rs-0038 */         SortedMap { data: Vec::new() }
/* FP:sorted_map.rs-0039 */     }
/* FP:sorted_map.rs-0040 */ }
/* FP:sorted_map.rs-0041 */ 
/* FP:sorted_map.rs-0042 */ impl<K: Ord, V> SortedMap<K, V> {
/* FP:sorted_map.rs-0043 */     /// Construct a `SortedMap` from a presorted set of elements. This is faster
/* FP:sorted_map.rs-0044 */     /// than creating an empty map and then inserting the elements individually.
/* FP:sorted_map.rs-0045 */     ///
/* FP:sorted_map.rs-0046 */     /// It is up to the caller to make sure that the elements are sorted by key
/* FP:sorted_map.rs-0047 */     /// and that there are no duplicates.
/* FP:sorted_map.rs-0048 */     #[inline]
/* FP:sorted_map.rs-0049 */     pub fn from_presorted_elements(elements: Vec<(K, V)>) -> SortedMap<K, V> {
/* FP:sorted_map.rs-0050 */         debug_assert!(elements.array_windows().all(|[fst, snd]| fst.0 < snd.0));
/* FP:sorted_map.rs-0051 */ 
/* FP:sorted_map.rs-0052 */         SortedMap { data: elements }
/* FP:sorted_map.rs-0053 */     }
/* FP:sorted_map.rs-0054 */ 
/* FP:sorted_map.rs-0055 */     #[inline]
/* FP:sorted_map.rs-0056 */     pub fn insert(&mut self, key: K, value: V) -> Option<V> {
/* FP:sorted_map.rs-0057 */         match self.lookup_index_for(&key) {
/* FP:sorted_map.rs-0058 */             Ok(index) => {
/* FP:sorted_map.rs-0059 */                 let slot = unsafe { self.data.get_unchecked_mut(index) };
/* FP:sorted_map.rs-0060 */                 Some(mem::replace(&mut slot.1, value))
/* FP:sorted_map.rs-0061 */             }
/* FP:sorted_map.rs-0062 */             Err(index) => {
/* FP:sorted_map.rs-0063 */                 self.data.insert(index, (key, value));
/* FP:sorted_map.rs-0064 */                 None
/* FP:sorted_map.rs-0065 */             }
/* FP:sorted_map.rs-0066 */         }
/* FP:sorted_map.rs-0067 */     }
/* FP:sorted_map.rs-0068 */ 
/* FP:sorted_map.rs-0069 */     #[inline]
/* FP:sorted_map.rs-0070 */     pub fn remove(&mut self, key: &K) -> Option<V> {
/* FP:sorted_map.rs-0071 */         match self.lookup_index_for(key) {
/* FP:sorted_map.rs-0072 */             Ok(index) => Some(self.data.remove(index).1),
/* FP:sorted_map.rs-0073 */             Err(_) => None,
/* FP:sorted_map.rs-0074 */         }
/* FP:sorted_map.rs-0075 */     }
/* FP:sorted_map.rs-0076 */ 
/* FP:sorted_map.rs-0077 */     #[inline]
/* FP:sorted_map.rs-0078 */     pub fn get<Q>(&self, key: &Q) -> Option<&V>
/* FP:sorted_map.rs-0079 */     where
/* FP:sorted_map.rs-0080 */         K: Borrow<Q>,
/* FP:sorted_map.rs-0081 */         Q: Ord + ?Sized,
/* FP:sorted_map.rs-0082 */     {
/* FP:sorted_map.rs-0083 */         match self.lookup_index_for(key) {
/* FP:sorted_map.rs-0084 */             Ok(index) => unsafe { Some(&self.data.get_unchecked(index).1) },
/* FP:sorted_map.rs-0085 */             Err(_) => None,
/* FP:sorted_map.rs-0086 */         }
/* FP:sorted_map.rs-0087 */     }
/* FP:sorted_map.rs-0088 */ 
/* FP:sorted_map.rs-0089 */     #[inline]
/* FP:sorted_map.rs-0090 */     pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
/* FP:sorted_map.rs-0091 */     where
/* FP:sorted_map.rs-0092 */         K: Borrow<Q>,
/* FP:sorted_map.rs-0093 */         Q: Ord + ?Sized,
/* FP:sorted_map.rs-0094 */     {
/* FP:sorted_map.rs-0095 */         match self.lookup_index_for(key) {
/* FP:sorted_map.rs-0096 */             Ok(index) => unsafe { Some(&mut self.data.get_unchecked_mut(index).1) },
/* FP:sorted_map.rs-0097 */             Err(_) => None,
/* FP:sorted_map.rs-0098 */         }
/* FP:sorted_map.rs-0099 */     }
/* FP:sorted_map.rs-0100 */ 
/* FP:sorted_map.rs-0101 */     /// Gets a mutable reference to the value in the entry, or insert a new one.
/* FP:sorted_map.rs-0102 */     #[inline]
/* FP:sorted_map.rs-0103 */     pub fn get_mut_or_insert_default(&mut self, key: K) -> &mut V
/* FP:sorted_map.rs-0104 */     where
/* FP:sorted_map.rs-0105 */         K: Eq,
/* FP:sorted_map.rs-0106 */         V: Default,
/* FP:sorted_map.rs-0107 */     {
/* FP:sorted_map.rs-0108 */         let index = match self.lookup_index_for(&key) {
/* FP:sorted_map.rs-0109 */             Ok(index) => index,
/* FP:sorted_map.rs-0110 */             Err(index) => {
/* FP:sorted_map.rs-0111 */                 self.data.insert(index, (key, V::default()));
/* FP:sorted_map.rs-0112 */                 index
/* FP:sorted_map.rs-0113 */             }
/* FP:sorted_map.rs-0114 */         };
/* FP:sorted_map.rs-0115 */         unsafe { &mut self.data.get_unchecked_mut(index).1 }
/* FP:sorted_map.rs-0116 */     }
/* FP:sorted_map.rs-0117 */ 
/* FP:sorted_map.rs-0118 */     #[inline]
/* FP:sorted_map.rs-0119 */     pub fn clear(&mut self) {
/* FP:sorted_map.rs-0120 */         self.data.clear();
/* FP:sorted_map.rs-0121 */     }
/* FP:sorted_map.rs-0122 */ 
/* FP:sorted_map.rs-0123 */     /// Iterate over elements, sorted by key
/* FP:sorted_map.rs-0124 */     #[inline]
/* FP:sorted_map.rs-0125 */     pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
/* FP:sorted_map.rs-0126 */         self.data.iter()
/* FP:sorted_map.rs-0127 */     }
/* FP:sorted_map.rs-0128 */ 
/* FP:sorted_map.rs-0129 */     /// Iterate over the keys, sorted
/* FP:sorted_map.rs-0130 */     #[inline]
/* FP:sorted_map.rs-0131 */     pub fn keys(&self) -> impl ExactSizeIterator<Item = &K> + DoubleEndedIterator {
/* FP:sorted_map.rs-0132 */         self.data.iter().map(|(k, _)| k)
/* FP:sorted_map.rs-0133 */     }
/* FP:sorted_map.rs-0134 */ 
/* FP:sorted_map.rs-0135 */     /// Iterate over values, sorted by key
/* FP:sorted_map.rs-0136 */     #[inline]
/* FP:sorted_map.rs-0137 */     pub fn values(&self) -> impl ExactSizeIterator<Item = &V> + DoubleEndedIterator {
/* FP:sorted_map.rs-0138 */         self.data.iter().map(|(_, v)| v)
/* FP:sorted_map.rs-0139 */     }
/* FP:sorted_map.rs-0140 */ 
/* FP:sorted_map.rs-0141 */     #[inline]
/* FP:sorted_map.rs-0142 */     pub fn len(&self) -> usize {
/* FP:sorted_map.rs-0143 */         self.data.len()
/* FP:sorted_map.rs-0144 */     }
/* FP:sorted_map.rs-0145 */ 
/* FP:sorted_map.rs-0146 */     #[inline]
/* FP:sorted_map.rs-0147 */     pub fn is_empty(&self) -> bool {
/* FP:sorted_map.rs-0148 */         self.len() == 0
/* FP:sorted_map.rs-0149 */     }
/* FP:sorted_map.rs-0150 */ 
/* FP:sorted_map.rs-0151 */     #[inline]
/* FP:sorted_map.rs-0152 */     pub fn range<R>(&self, range: R) -> &[(K, V)]
/* FP:sorted_map.rs-0153 */     where
/* FP:sorted_map.rs-0154 */         R: RangeBounds<K>,
/* FP:sorted_map.rs-0155 */     {
/* FP:sorted_map.rs-0156 */         let (start, end) = self.range_slice_indices(range);
/* FP:sorted_map.rs-0157 */         &self.data[start..end]
/* FP:sorted_map.rs-0158 */     }
/* FP:sorted_map.rs-0159 */ 
/* FP:sorted_map.rs-0160 */     /// `sm.range_is_empty(r)` == `sm.range(r).is_empty()`, but is faster.
/* FP:sorted_map.rs-0161 */     #[inline]
/* FP:sorted_map.rs-0162 */     pub fn range_is_empty<R>(&self, range: R) -> bool
/* FP:sorted_map.rs-0163 */     where
/* FP:sorted_map.rs-0164 */         R: RangeBounds<K>,
/* FP:sorted_map.rs-0165 */     {
/* FP:sorted_map.rs-0166 */         // `range` must (via `range_slice_indices`) search for the start and
/* FP:sorted_map.rs-0167 */         // end separately. But here we can do a single binary search for the
/* FP:sorted_map.rs-0168 */         // entire range. If a single `x` matching `range` is found then the
/* FP:sorted_map.rs-0169 */         // range is *not* empty.
/* FP:sorted_map.rs-0170 */         self.data
/* FP:sorted_map.rs-0171 */             .binary_search_by(|(x, _)| {
/* FP:sorted_map.rs-0172 */                 // Is `x` below `range`?
/* FP:sorted_map.rs-0173 */                 match range.start_bound() {
/* FP:sorted_map.rs-0174 */                     Bound::Included(start) if x < start => return Ordering::Less,
/* FP:sorted_map.rs-0175 */                     Bound::Excluded(start) if x <= start => return Ordering::Less,
/* FP:sorted_map.rs-0176 */                     _ => {}
/* FP:sorted_map.rs-0177 */                 };
/* FP:sorted_map.rs-0178 */ 
/* FP:sorted_map.rs-0179 */                 // Is `x` above `range`?
/* FP:sorted_map.rs-0180 */                 match range.end_bound() {
/* FP:sorted_map.rs-0181 */                     Bound::Included(end) if x > end => return Ordering::Greater,
/* FP:sorted_map.rs-0182 */                     Bound::Excluded(end) if x >= end => return Ordering::Greater,
/* FP:sorted_map.rs-0183 */                     _ => {}
/* FP:sorted_map.rs-0184 */                 };
/* FP:sorted_map.rs-0185 */ 
/* FP:sorted_map.rs-0186 */                 // `x` must be within `range`.
/* FP:sorted_map.rs-0187 */                 Ordering::Equal
/* FP:sorted_map.rs-0188 */             })
/* FP:sorted_map.rs-0189 */             .is_err()
/* FP:sorted_map.rs-0190 */     }
/* FP:sorted_map.rs-0191 */ 
/* FP:sorted_map.rs-0192 */     #[inline]
/* FP:sorted_map.rs-0193 */     pub fn remove_range<R>(&mut self, range: R)
/* FP:sorted_map.rs-0194 */     where
/* FP:sorted_map.rs-0195 */         R: RangeBounds<K>,
/* FP:sorted_map.rs-0196 */     {
/* FP:sorted_map.rs-0197 */         let (start, end) = self.range_slice_indices(range);
/* FP:sorted_map.rs-0198 */         self.data.splice(start..end, std::iter::empty());
/* FP:sorted_map.rs-0199 */     }
/* FP:sorted_map.rs-0200 */ 
/* FP:sorted_map.rs-0201 */     /// Mutate all keys with the given function `f`. This mutation must not
/* FP:sorted_map.rs-0202 */     /// change the sort-order of keys.
/* FP:sorted_map.rs-0203 */     #[inline]
/* FP:sorted_map.rs-0204 */     pub fn offset_keys<F>(&mut self, f: F)
/* FP:sorted_map.rs-0205 */     where
/* FP:sorted_map.rs-0206 */         F: Fn(&mut K),
/* FP:sorted_map.rs-0207 */     {
/* FP:sorted_map.rs-0208 */         self.data.iter_mut().map(|(k, _)| k).for_each(f);
/* FP:sorted_map.rs-0209 */     }
/* FP:sorted_map.rs-0210 */ 
/* FP:sorted_map.rs-0211 */     /// Inserts a presorted range of elements into the map. If the range can be
/* FP:sorted_map.rs-0212 */     /// inserted as a whole in between to existing elements of the map, this
/* FP:sorted_map.rs-0213 */     /// will be faster than inserting the elements individually.
/* FP:sorted_map.rs-0214 */     ///
/* FP:sorted_map.rs-0215 */     /// It is up to the caller to make sure that the elements are sorted by key
/* FP:sorted_map.rs-0216 */     /// and that there are no duplicates.
/* FP:sorted_map.rs-0217 */     #[inline]
/* FP:sorted_map.rs-0218 */     pub fn insert_presorted(&mut self, elements: Vec<(K, V)>) {
/* FP:sorted_map.rs-0219 */         if elements.is_empty() {
/* FP:sorted_map.rs-0220 */             return;
/* FP:sorted_map.rs-0221 */         }
/* FP:sorted_map.rs-0222 */ 
/* FP:sorted_map.rs-0223 */         debug_assert!(elements.array_windows().all(|[fst, snd]| fst.0 < snd.0));
/* FP:sorted_map.rs-0224 */ 
/* FP:sorted_map.rs-0225 */         let start_index = self.lookup_index_for(&elements[0].0);
/* FP:sorted_map.rs-0226 */ 
/* FP:sorted_map.rs-0227 */         let elements = match start_index {
/* FP:sorted_map.rs-0228 */             Ok(index) => {
/* FP:sorted_map.rs-0229 */                 let mut elements = elements.into_iter();
/* FP:sorted_map.rs-0230 */                 self.data[index] = elements.next().unwrap();
/* FP:sorted_map.rs-0231 */                 elements
/* FP:sorted_map.rs-0232 */             }
/* FP:sorted_map.rs-0233 */             Err(index) => {
/* FP:sorted_map.rs-0234 */                 if index == self.data.len() || elements.last().unwrap().0 < self.data[index].0 {
/* FP:sorted_map.rs-0235 */                     // We can copy the whole range without having to mix with
/* FP:sorted_map.rs-0236 */                     // existing elements.
/* FP:sorted_map.rs-0237 */                     self.data.splice(index..index, elements);
/* FP:sorted_map.rs-0238 */                     return;
/* FP:sorted_map.rs-0239 */                 }
/* FP:sorted_map.rs-0240 */ 
/* FP:sorted_map.rs-0241 */                 let mut elements = elements.into_iter();
/* FP:sorted_map.rs-0242 */                 self.data.insert(index, elements.next().unwrap());
/* FP:sorted_map.rs-0243 */                 elements
/* FP:sorted_map.rs-0244 */             }
/* FP:sorted_map.rs-0245 */         };
/* FP:sorted_map.rs-0246 */ 
/* FP:sorted_map.rs-0247 */         // Insert the rest
/* FP:sorted_map.rs-0248 */         for (k, v) in elements {
/* FP:sorted_map.rs-0249 */             self.insert(k, v);
/* FP:sorted_map.rs-0250 */         }
/* FP:sorted_map.rs-0251 */     }
/* FP:sorted_map.rs-0252 */ 
/* FP:sorted_map.rs-0253 */     /// Looks up the key in `self.data` via `slice::binary_search()`.
/* FP:sorted_map.rs-0254 */     #[inline(always)]
/* FP:sorted_map.rs-0255 */     fn lookup_index_for<Q>(&self, key: &Q) -> Result<usize, usize>
/* FP:sorted_map.rs-0256 */     where
/* FP:sorted_map.rs-0257 */         K: Borrow<Q>,
/* FP:sorted_map.rs-0258 */         Q: Ord + ?Sized,
/* FP:sorted_map.rs-0259 */     {
/* FP:sorted_map.rs-0260 */         self.data.binary_search_by(|(x, _)| x.borrow().cmp(key))
/* FP:sorted_map.rs-0261 */     }
/* FP:sorted_map.rs-0262 */ 
/* FP:sorted_map.rs-0263 */     #[inline]
/* FP:sorted_map.rs-0264 */     fn range_slice_indices<R>(&self, range: R) -> (usize, usize)
/* FP:sorted_map.rs-0265 */     where
/* FP:sorted_map.rs-0266 */         R: RangeBounds<K>,
/* FP:sorted_map.rs-0267 */     {
/* FP:sorted_map.rs-0268 */         let start = match range.start_bound() {
/* FP:sorted_map.rs-0269 */             Bound::Included(k) => match self.lookup_index_for(k) {
/* FP:sorted_map.rs-0270 */                 Ok(index) | Err(index) => index,
/* FP:sorted_map.rs-0271 */             },
/* FP:sorted_map.rs-0272 */             Bound::Excluded(k) => match self.lookup_index_for(k) {
/* FP:sorted_map.rs-0273 */                 Ok(index) => index + 1,
/* FP:sorted_map.rs-0274 */                 Err(index) => index,
/* FP:sorted_map.rs-0275 */             },
/* FP:sorted_map.rs-0276 */             Bound::Unbounded => 0,
/* FP:sorted_map.rs-0277 */         };
/* FP:sorted_map.rs-0278 */ 
/* FP:sorted_map.rs-0279 */         let end = match range.end_bound() {
/* FP:sorted_map.rs-0280 */             Bound::Included(k) => match self.lookup_index_for(k) {
/* FP:sorted_map.rs-0281 */                 Ok(index) => index + 1,
/* FP:sorted_map.rs-0282 */                 Err(index) => index,
/* FP:sorted_map.rs-0283 */             },
/* FP:sorted_map.rs-0284 */             Bound::Excluded(k) => match self.lookup_index_for(k) {
/* FP:sorted_map.rs-0285 */                 Ok(index) | Err(index) => index,
/* FP:sorted_map.rs-0286 */             },
/* FP:sorted_map.rs-0287 */             Bound::Unbounded => self.data.len(),
/* FP:sorted_map.rs-0288 */         };
/* FP:sorted_map.rs-0289 */ 
/* FP:sorted_map.rs-0290 */         (start, end)
/* FP:sorted_map.rs-0291 */     }
/* FP:sorted_map.rs-0292 */ 
/* FP:sorted_map.rs-0293 */     #[inline]
/* FP:sorted_map.rs-0294 */     pub fn contains_key<Q>(&self, key: &Q) -> bool
/* FP:sorted_map.rs-0295 */     where
/* FP:sorted_map.rs-0296 */         K: Borrow<Q>,
/* FP:sorted_map.rs-0297 */         Q: Ord + ?Sized,
/* FP:sorted_map.rs-0298 */     {
/* FP:sorted_map.rs-0299 */         self.get(key).is_some()
/* FP:sorted_map.rs-0300 */     }
/* FP:sorted_map.rs-0301 */ }
/* FP:sorted_map.rs-0302 */ 
/* FP:sorted_map.rs-0303 */ impl<K: Ord, V> IntoIterator for SortedMap<K, V> {
/* FP:sorted_map.rs-0304 */     type Item = (K, V);
/* FP:sorted_map.rs-0305 */     type IntoIter = std::vec::IntoIter<(K, V)>;
/* FP:sorted_map.rs-0306 */ 
/* FP:sorted_map.rs-0307 */     fn into_iter(self) -> Self::IntoIter {
/* FP:sorted_map.rs-0308 */         self.data.into_iter()
/* FP:sorted_map.rs-0309 */     }
/* FP:sorted_map.rs-0310 */ }
/* FP:sorted_map.rs-0311 */ 
/* FP:sorted_map.rs-0312 */ impl<'a, K, Q, V> Index<&'a Q> for SortedMap<K, V>
/* FP:sorted_map.rs-0313 */ where
/* FP:sorted_map.rs-0314 */     K: Ord + Borrow<Q>,
/* FP:sorted_map.rs-0315 */     Q: Ord + ?Sized,
/* FP:sorted_map.rs-0316 */ {
/* FP:sorted_map.rs-0317 */     type Output = V;
/* FP:sorted_map.rs-0318 */ 
/* FP:sorted_map.rs-0319 */     fn index(&self, key: &Q) -> &Self::Output {
/* FP:sorted_map.rs-0320 */         self.get(key).expect("no entry found for key")
/* FP:sorted_map.rs-0321 */     }
/* FP:sorted_map.rs-0322 */ }
/* FP:sorted_map.rs-0323 */ 
/* FP:sorted_map.rs-0324 */ impl<'a, K, Q, V> IndexMut<&'a Q> for SortedMap<K, V>
/* FP:sorted_map.rs-0325 */ where
/* FP:sorted_map.rs-0326 */     K: Ord + Borrow<Q>,
/* FP:sorted_map.rs-0327 */     Q: Ord + ?Sized,
/* FP:sorted_map.rs-0328 */ {
/* FP:sorted_map.rs-0329 */     fn index_mut(&mut self, key: &Q) -> &mut Self::Output {
/* FP:sorted_map.rs-0330 */         self.get_mut(key).expect("no entry found for key")
/* FP:sorted_map.rs-0331 */     }
/* FP:sorted_map.rs-0332 */ }
/* FP:sorted_map.rs-0333 */ 
/* FP:sorted_map.rs-0334 */ impl<K: Ord, V> FromIterator<(K, V)> for SortedMap<K, V> {
/* FP:sorted_map.rs-0335 */     fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
/* FP:sorted_map.rs-0336 */         let mut data: Vec<(K, V)> = iter.into_iter().collect();
/* FP:sorted_map.rs-0337 */ 
/* FP:sorted_map.rs-0338 */         data.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
/* FP:sorted_map.rs-0339 */         data.dedup_by(|(k1, _), (k2, _)| k1 == k2);
/* FP:sorted_map.rs-0340 */ 
/* FP:sorted_map.rs-0341 */         SortedMap { data }
/* FP:sorted_map.rs-0342 */     }
/* FP:sorted_map.rs-0343 */ }
/* FP:sorted_map.rs-0344 */ 
/* FP:sorted_map.rs-0345 */ impl<K: HashStable<CTX> + StableOrd, V: HashStable<CTX>, CTX> HashStable<CTX> for SortedMap<K, V> {
/* FP:sorted_map.rs-0346 */     #[inline]
/* FP:sorted_map.rs-0347 */     fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
/* FP:sorted_map.rs-0348 */         self.data.hash_stable(ctx, hasher);
/* FP:sorted_map.rs-0349 */     }
/* FP:sorted_map.rs-0350 */ }
/* FP:sorted_map.rs-0351 */ 
/* FP:sorted_map.rs-0352 */ impl<K: Debug, V: Debug> Debug for SortedMap<K, V> {
/* FP:sorted_map.rs-0353 */     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:sorted_map.rs-0354 */         f.debug_map().entries(self.data.iter().map(|(a, b)| (a, b))).finish()
/* FP:sorted_map.rs-0355 */     }
/* FP:sorted_map.rs-0356 */ }
/* FP:sorted_map.rs-0357 */ 
/* FP:sorted_map.rs-0358 */ #[cfg(test)]