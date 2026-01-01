/* FP:lib.rs-0001 */ // The arena, a fast but limited type of allocator.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // Arenas are a type of allocator that destroy the objects within, all at
/* FP:lib.rs-0004 */ // once, once the arena itself is destroyed. They do not support deallocation
/* FP:lib.rs-0005 */ // of individual objects while the arena itself is still alive. The benefit
/* FP:lib.rs-0006 */ // of an arena is very fast allocation; just a pointer bump.
/* FP:lib.rs-0007 */ //
/* FP:lib.rs-0008 */ // This crate implements several kinds of arena.
/* FP:lib.rs-0009 */ 
/* FP:lib.rs-0010 */ // tidy-alphabetical-start
/* FP:lib.rs-0011 */ #[allow(clippy::mut_from_ref)] // Arena allocators are one place where this pattern is fine.
/* FP:lib.rs-0012 */ #[allow(internal_features)]
/* FP:lib.rs-0013 */ #[cfg_attr(test, feature(test))]
/* FP:lib.rs-0014 */ #[deny(unsafe_op_in_unsafe_fn)]
/* FP:lib.rs-0015 */ #[doc(
/* FP:lib.rs-0016 */     html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/",
/* FP:lib.rs-0017 */     test(no_crate_inject, attr(deny(warnings)))
/* FP:lib.rs-0018 */ )]
/* FP:lib.rs-0019 */ #[doc(rust_logo)]
/* FP:lib.rs-0020 */ #[feature(core_intrinsics)]
/* FP:lib.rs-0021 */ #[feature(decl_macro)]
/* FP:lib.rs-0022 */ #[feature(dropck_eyepatch)]
/* FP:lib.rs-0023 */ #[feature(maybe_uninit_slice)]
/* FP:lib.rs-0024 */ #[feature(never_type)]
/* FP:lib.rs-0025 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0026 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0027 */ #[feature(unwrap_infallible)]
/* FP:lib.rs-0028 */ // tidy-alphabetical-end
/* FP:lib.rs-0029 */ 
/* FP:lib.rs-0030 */ use std::alloc::Layout;
/* FP:lib.rs-0031 */ use std::cell::{Cell, RefCell};
/* FP:lib.rs-0032 */ use std::marker::PhantomData;
/* FP:lib.rs-0033 */ use std::mem::{self, MaybeUninit};
/* FP:lib.rs-0034 */ use std::ptr::{self, NonNull};
/* FP:lib.rs-0035 */ use std::{cmp, intrinsics, slice};
/* FP:lib.rs-0036 */ 
/* FP:lib.rs-0037 */ use smallvec::SmallVec;
/* FP:lib.rs-0038 */ 
/* FP:lib.rs-0039 */ /// This calls the passed function while ensuring it won't be inlined into the caller.
/* FP:lib.rs-0040 */ #[inline(never)]
/* FP:lib.rs-0041 */ #[cold]
/* FP:lib.rs-0042 */ fn outline<F: FnOnce() -> R, R>(f: F) -> R {
/* FP:lib.rs-0043 */     f()
/* FP:lib.rs-0044 */ }
/* FP:lib.rs-0045 */ 
/* FP:lib.rs-0046 */ struct ArenaChunk<T = u8> {
/* FP:lib.rs-0047 */     /// The raw storage for the arena chunk.
/* FP:lib.rs-0048 */     storage: NonNull<[MaybeUninit<T>]>,
/* FP:lib.rs-0049 */     /// The number of valid entries in the chunk.
/* FP:lib.rs-0050 */     entries: usize,
/* FP:lib.rs-0051 */ }
/* FP:lib.rs-0052 */ 
/* FP:lib.rs-0053 */ unsafe impl<#[may_dangle] T> Drop for ArenaChunk<T> {
/* FP:lib.rs-0054 */     fn drop(&mut self) {
/* FP:lib.rs-0055 */         unsafe { drop(Box::from_raw(self.storage.as_mut())) }
/* FP:lib.rs-0056 */     }
/* FP:lib.rs-0057 */ }
/* FP:lib.rs-0058 */ 
/* FP:lib.rs-0059 */ impl<T> ArenaChunk<T> {
/* FP:lib.rs-0060 */     #[inline]
/* FP:lib.rs-0061 */     unsafe fn new(capacity: usize) -> ArenaChunk<T> {
/* FP:lib.rs-0062 */         ArenaChunk {
/* FP:lib.rs-0063 */             storage: NonNull::from(Box::leak(Box::new_uninit_slice(capacity))),
/* FP:lib.rs-0064 */             entries: 0,
/* FP:lib.rs-0065 */         }
/* FP:lib.rs-0066 */     }
/* FP:lib.rs-0067 */ 
/* FP:lib.rs-0068 */     /// Destroys this arena chunk.
/* FP:lib.rs-0069 */     ///
/* FP:lib.rs-0070 */     /// # Safety
/* FP:lib.rs-0071 */     ///
/* FP:lib.rs-0072 */     /// The caller must ensure that `len` elements of this chunk have been initialized.
/* FP:lib.rs-0073 */     #[inline]
/* FP:lib.rs-0074 */     unsafe fn destroy(&mut self, len: usize) {
/* FP:lib.rs-0075 */         // The branch on needs_drop() is an -O1 performance optimization.
/* FP:lib.rs-0076 */         // Without the branch, dropping TypedArena<T> takes linear time.
/* FP:lib.rs-0077 */         if mem::needs_drop::<T>() {
/* FP:lib.rs-0078 */             // SAFETY: The caller must ensure that `len` elements of this chunk have
/* FP:lib.rs-0079 */             // been initialized.
/* FP:lib.rs-0080 */             unsafe {
/* FP:lib.rs-0081 */                 let slice = self.storage.as_mut();
/* FP:lib.rs-0082 */                 slice[..len].assume_init_drop();
/* FP:lib.rs-0083 */             }
/* FP:lib.rs-0084 */         }
/* FP:lib.rs-0085 */     }
/* FP:lib.rs-0086 */ 
/* FP:lib.rs-0087 */     // Returns a pointer to the first allocated object.
/* FP:lib.rs-0088 */     #[inline]
/* FP:lib.rs-0089 */     fn start(&mut self) -> *mut T {
/* FP:lib.rs-0090 */         self.storage.as_ptr() as *mut T
/* FP:lib.rs-0091 */     }
/* FP:lib.rs-0092 */ 
/* FP:lib.rs-0093 */     // Returns a pointer to the end of the allocated space.
/* FP:lib.rs-0094 */     #[inline]
/* FP:lib.rs-0095 */     fn end(&mut self) -> *mut T {
/* FP:lib.rs-0096 */         unsafe {
/* FP:lib.rs-0097 */             if size_of::<T>() == 0 {
/* FP:lib.rs-0098 */                 // A pointer as large as possible for zero-sized elements.
/* FP:lib.rs-0099 */                 ptr::without_provenance_mut(!0)
/* FP:lib.rs-0100 */             } else {
/* FP:lib.rs-0101 */                 self.start().add(self.storage.len())
/* FP:lib.rs-0102 */             }
/* FP:lib.rs-0103 */         }
/* FP:lib.rs-0104 */     }
/* FP:lib.rs-0105 */ }
/* FP:lib.rs-0106 */ 
/* FP:lib.rs-0107 */ // The arenas start with PAGE-sized chunks, and then each new chunk is twice as
/* FP:lib.rs-0108 */ // big as its predecessor, up until we reach HUGE_PAGE-sized chunks, whereupon
/* FP:lib.rs-0109 */ // we stop growing. This scales well, from arenas that are barely used up to
/* FP:lib.rs-0110 */ // arenas that are used for 100s of MiBs. Note also that the chosen sizes match
/* FP:lib.rs-0111 */ // the usual sizes of pages and huge pages on Linux.
/* FP:lib.rs-0112 */ const PAGE: usize = 4096;
/* FP:lib.rs-0113 */ const HUGE_PAGE: usize = 2 * 1024 * 1024;
/* FP:lib.rs-0114 */ 
/* FP:lib.rs-0115 */ /// An arena that can hold objects of only one type.
/* FP:lib.rs-0116 */ pub struct TypedArena<T> {
/* FP:lib.rs-0117 */     /// A pointer to the next object to be allocated.
/* FP:lib.rs-0118 */     ptr: Cell<*mut T>,
/* FP:lib.rs-0119 */ 
/* FP:lib.rs-0120 */     /// A pointer to the end of the allocated area. When this pointer is
/* FP:lib.rs-0121 */     /// reached, a new chunk is allocated.
/* FP:lib.rs-0122 */     end: Cell<*mut T>,
/* FP:lib.rs-0123 */ 
/* FP:lib.rs-0124 */     /// A vector of arena chunks.
/* FP:lib.rs-0125 */     chunks: RefCell<Vec<ArenaChunk<T>>>,
/* FP:lib.rs-0126 */ 
/* FP:lib.rs-0127 */     /// Marker indicating that dropping the arena causes its owned
/* FP:lib.rs-0128 */     /// instances of `T` to be dropped.
/* FP:lib.rs-0129 */     _own: PhantomData<T>,
/* FP:lib.rs-0130 */ }
/* FP:lib.rs-0131 */ 
/* FP:lib.rs-0132 */ impl<T> Default for TypedArena<T> {
/* FP:lib.rs-0133 */     /// Creates a new `TypedArena`.
/* FP:lib.rs-0134 */     fn default() -> TypedArena<T> {
/* FP:lib.rs-0135 */         TypedArena {
/* FP:lib.rs-0136 */             // We set both `ptr` and `end` to 0 so that the first call to
/* FP:lib.rs-0137 */             // alloc() will trigger a grow().
/* FP:lib.rs-0138 */             ptr: Cell::new(ptr::null_mut()),
/* FP:lib.rs-0139 */             end: Cell::new(ptr::null_mut()),
/* FP:lib.rs-0140 */             chunks: Default::default(),
/* FP:lib.rs-0141 */             _own: PhantomData,
/* FP:lib.rs-0142 */         }
/* FP:lib.rs-0143 */     }
/* FP:lib.rs-0144 */ }
/* FP:lib.rs-0145 */ 
/* FP:lib.rs-0146 */ impl<T> TypedArena<T> {
/* FP:lib.rs-0147 */     /// Allocates an object in the `TypedArena`, returning a reference to it.
/* FP:lib.rs-0148 */     #[inline]
/* FP:lib.rs-0149 */     pub fn alloc(&self, object: T) -> &mut T {
/* FP:lib.rs-0150 */         if self.ptr == self.end {
/* FP:lib.rs-0151 */             self.grow(1)
/* FP:lib.rs-0152 */         }
/* FP:lib.rs-0153 */ 
/* FP:lib.rs-0154 */         unsafe {
/* FP:lib.rs-0155 */             if size_of::<T>() == 0 {
/* FP:lib.rs-0156 */                 self.ptr.set(self.ptr.get().wrapping_byte_add(1));
/* FP:lib.rs-0157 */                 let ptr = ptr::NonNull::<T>::dangling().as_ptr();
/* FP:lib.rs-0158 */                 // Don't drop the object. This `write` is equivalent to `forget`.
/* FP:lib.rs-0159 */                 ptr::write(ptr, object);
/* FP:lib.rs-0160 */                 &mut *ptr
/* FP:lib.rs-0161 */             } else {
/* FP:lib.rs-0162 */                 let ptr = self.ptr.get();
/* FP:lib.rs-0163 */                 // Advance the pointer.
/* FP:lib.rs-0164 */                 self.ptr.set(self.ptr.get().add(1));
/* FP:lib.rs-0165 */                 // Write into uninitialized memory.
/* FP:lib.rs-0166 */                 ptr::write(ptr, object);
/* FP:lib.rs-0167 */                 &mut *ptr
/* FP:lib.rs-0168 */             }
/* FP:lib.rs-0169 */         }
/* FP:lib.rs-0170 */     }
/* FP:lib.rs-0171 */ 
/* FP:lib.rs-0172 */     #[inline]
/* FP:lib.rs-0173 */     fn can_allocate(&self, additional: usize) -> bool {
/* FP:lib.rs-0174 */         // FIXME: this should *likely* use `offset_from`, but more
/* FP:lib.rs-0175 */         // investigation is needed (including running tests in miri).
/* FP:lib.rs-0176 */         let available_bytes = self.end.get().addr() - self.ptr.get().addr();
/* FP:lib.rs-0177 */         let additional_bytes = additional.checked_mul(size_of::<T>()).unwrap();
/* FP:lib.rs-0178 */         available_bytes >= additional_bytes
/* FP:lib.rs-0179 */     }
/* FP:lib.rs-0180 */ 
/* FP:lib.rs-0181 */     #[inline]
/* FP:lib.rs-0182 */     fn alloc_raw_slice(&self, len: usize) -> *mut T {
/* FP:lib.rs-0183 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0184 */         assert!(len != 0);
/* FP:lib.rs-0185 */ 
/* FP:lib.rs-0186 */         // Ensure the current chunk can fit `len` objects.
/* FP:lib.rs-0187 */         if !self.can_allocate(len) {
/* FP:lib.rs-0188 */             self.grow(len);
/* FP:lib.rs-0189 */             debug_assert!(self.can_allocate(len));
/* FP:lib.rs-0190 */         }
/* FP:lib.rs-0191 */ 
/* FP:lib.rs-0192 */         let start_ptr = self.ptr.get();
/* FP:lib.rs-0193 */         // SAFETY: `can_allocate`/`grow` ensures that there is enough space for
/* FP:lib.rs-0194 */         // `len` elements.
/* FP:lib.rs-0195 */         unsafe { self.ptr.set(start_ptr.add(len)) };
/* FP:lib.rs-0196 */         start_ptr
/* FP:lib.rs-0197 */     }
/* FP:lib.rs-0198 */ 
/* FP:lib.rs-0199 */     /// Allocates the elements of this iterator into a contiguous slice in the `TypedArena`.
/* FP:lib.rs-0200 */     ///
/* FP:lib.rs-0201 */     /// Note: for reasons of reentrancy and panic safety we collect into a `SmallVec<[_; 8]>` before
/* FP:lib.rs-0202 */     /// storing the elements in the arena.
/* FP:lib.rs-0203 */     #[inline]
/* FP:lib.rs-0204 */     pub fn alloc_from_iter<I: IntoIterator<Item = T>>(&self, iter: I) -> &mut [T] {
/* FP:lib.rs-0205 */         self.try_alloc_from_iter(iter.into_iter().map(Ok::<T, !>)).into_ok()
/* FP:lib.rs-0206 */     }
/* FP:lib.rs-0207 */ 
/* FP:lib.rs-0208 */     /// Allocates the elements of this iterator into a contiguous slice in the `TypedArena`.
/* FP:lib.rs-0209 */     ///
/* FP:lib.rs-0210 */     /// Note: for reasons of reentrancy and panic safety we collect into a `SmallVec<[_; 8]>` before
/* FP:lib.rs-0211 */     /// storing the elements in the arena.
/* FP:lib.rs-0212 */     #[inline]
/* FP:lib.rs-0213 */     pub fn try_alloc_from_iter<E>(
/* FP:lib.rs-0214 */         &self,
/* FP:lib.rs-0215 */         iter: impl IntoIterator<Item = Result<T, E>>,
/* FP:lib.rs-0216 */     ) -> Result<&mut [T], E> {
/* FP:lib.rs-0217 */         // Despite the similarlty with `DroplessArena`, we cannot reuse their fast case. The reason
/* FP:lib.rs-0218 */         // is subtle: these arenas are reentrant. In other words, `iter` may very well be holding a
/* FP:lib.rs-0219 */         // reference to `self` and adding elements to the arena during iteration.
/* FP:lib.rs-0220 */         //
/* FP:lib.rs-0221 */         // For this reason, if we pre-allocated any space for the elements of this iterator, we'd
/* FP:lib.rs-0222 */         // have to track that some uninitialized elements are followed by some initialized elements,
/* FP:lib.rs-0223 */         // else we might accidentally drop uninitialized memory if something panics or if the
/* FP:lib.rs-0224 */         // iterator doesn't fill all the length we expected.
/* FP:lib.rs-0225 */         //
/* FP:lib.rs-0226 */         // So we collect all the elements beforehand, which takes care of reentrancy and panic
/* FP:lib.rs-0227 */         // safety. This function is much less hot than `DroplessArena::alloc_from_iter`, so it
/* FP:lib.rs-0228 */         // doesn't need to be hyper-optimized.
/* FP:lib.rs-0229 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0230 */ 
/* FP:lib.rs-0231 */         let vec: Result<SmallVec<[T; 8]>, E> = iter.into_iter().collect();
/* FP:lib.rs-0232 */         let mut vec = vec?;
/* FP:lib.rs-0233 */         if vec.is_empty() {
/* FP:lib.rs-0234 */             return Ok(&mut []);
/* FP:lib.rs-0235 */         }
/* FP:lib.rs-0236 */         // Move the content to the arena by copying and then forgetting it.
/* FP:lib.rs-0237 */         let len = vec.len();
/* FP:lib.rs-0238 */         let start_ptr = self.alloc_raw_slice(len);
/* FP:lib.rs-0239 */         Ok(unsafe {
/* FP:lib.rs-0240 */             vec.as_ptr().copy_to_nonoverlapping(start_ptr, len);
/* FP:lib.rs-0241 */             vec.set_len(0);
/* FP:lib.rs-0242 */             slice::from_raw_parts_mut(start_ptr, len)
/* FP:lib.rs-0243 */         })
/* FP:lib.rs-0244 */     }
/* FP:lib.rs-0245 */ 
/* FP:lib.rs-0246 */     /// Grows the arena.
/* FP:lib.rs-0247 */     #[inline(never)]
/* FP:lib.rs-0248 */     #[cold]
/* FP:lib.rs-0249 */     fn grow(&self, additional: usize) {
/* FP:lib.rs-0250 */         unsafe {
/* FP:lib.rs-0251 */             // We need the element size to convert chunk sizes (ranging from
/* FP:lib.rs-0252 */             // PAGE to HUGE_PAGE bytes) to element counts.
/* FP:lib.rs-0253 */             let elem_size = cmp::max(1, size_of::<T>());
/* FP:lib.rs-0254 */             let mut chunks = self.chunks.borrow_mut();
/* FP:lib.rs-0255 */             let mut new_cap;
/* FP:lib.rs-0256 */             if let Some(last_chunk) = chunks.last_mut() {
/* FP:lib.rs-0257 */                 // If a type is `!needs_drop`, we don't need to keep track of how many elements
/* FP:lib.rs-0258 */                 // the chunk stores - the field will be ignored anyway.
/* FP:lib.rs-0259 */                 if mem::needs_drop::<T>() {
/* FP:lib.rs-0260 */                     // FIXME: this should *likely* use `offset_from`, but more
/* FP:lib.rs-0261 */                     // investigation is needed (including running tests in miri).
/* FP:lib.rs-0262 */                     let used_bytes = self.ptr.get().addr() - last_chunk.start().addr();
/* FP:lib.rs-0263 */                     last_chunk.entries = used_bytes / size_of::<T>();
/* FP:lib.rs-0264 */                 }
/* FP:lib.rs-0265 */ 
/* FP:lib.rs-0266 */                 // If the previous chunk's len is less than HUGE_PAGE
/* FP:lib.rs-0267 */                 // bytes, then this chunk will be least double the previous
/* FP:lib.rs-0268 */                 // chunk's size.
/* FP:lib.rs-0269 */                 new_cap = last_chunk.storage.len().min(HUGE_PAGE / elem_size / 2);
/* FP:lib.rs-0270 */                 new_cap *= 2;
/* FP:lib.rs-0271 */             } else {
/* FP:lib.rs-0272 */                 new_cap = PAGE / elem_size;
/* FP:lib.rs-0273 */             }
/* FP:lib.rs-0274 */             // Also ensure that this chunk can fit `additional`.
/* FP:lib.rs-0275 */             new_cap = cmp::max(additional, new_cap);
/* FP:lib.rs-0276 */ 
/* FP:lib.rs-0277 */             let mut chunk = ArenaChunk::<T>::new(new_cap);
/* FP:lib.rs-0278 */             self.ptr.set(chunk.start());
/* FP:lib.rs-0279 */             self.end.set(chunk.end());
/* FP:lib.rs-0280 */             chunks.push(chunk);
/* FP:lib.rs-0281 */         }
/* FP:lib.rs-0282 */     }
/* FP:lib.rs-0283 */ 
/* FP:lib.rs-0284 */     // Drops the contents of the last chunk. The last chunk is partially empty, unlike all other
/* FP:lib.rs-0285 */     // chunks.
/* FP:lib.rs-0286 */     fn clear_last_chunk(&self, last_chunk: &mut ArenaChunk<T>) {
/* FP:lib.rs-0287 */         // Determine how much was filled.
/* FP:lib.rs-0288 */         let start = last_chunk.start().addr();
/* FP:lib.rs-0289 */         // We obtain the value of the pointer to the first uninitialized element.
/* FP:lib.rs-0290 */         let end = self.ptr.get().addr();
/* FP:lib.rs-0291 */         // We then calculate the number of elements to be dropped in the last chunk,
/* FP:lib.rs-0292 */         // which is the filled area's length.
/* FP:lib.rs-0293 */         let diff = if size_of::<T>() == 0 {
/* FP:lib.rs-0294 */             // `T` is ZST. It can't have a drop flag, so the value here doesn't matter. We get
/* FP:lib.rs-0295 */             // the number of zero-sized values in the last and only chunk, just out of caution.
/* FP:lib.rs-0296 */             // Recall that `end` was incremented for each allocated value.
/* FP:lib.rs-0297 */             end - start
/* FP:lib.rs-0298 */         } else {
/* FP:lib.rs-0299 */             // FIXME: this should *likely* use `offset_from`, but more
/* FP:lib.rs-0300 */             // investigation is needed (including running tests in miri).
/* FP:lib.rs-0301 */             (end - start) / size_of::<T>()
/* FP:lib.rs-0302 */         };
/* FP:lib.rs-0303 */         // Pass that to the `destroy` method.
/* FP:lib.rs-0304 */         unsafe {
/* FP:lib.rs-0305 */             last_chunk.destroy(diff);
/* FP:lib.rs-0306 */         }
/* FP:lib.rs-0307 */         // Reset the chunk.
/* FP:lib.rs-0308 */         self.ptr.set(last_chunk.start());
/* FP:lib.rs-0309 */     }
/* FP:lib.rs-0310 */ }
/* FP:lib.rs-0311 */ 
/* FP:lib.rs-0312 */ unsafe impl<#[may_dangle] T> Drop for TypedArena<T> {
/* FP:lib.rs-0313 */     fn drop(&mut self) {
/* FP:lib.rs-0314 */         unsafe {
/* FP:lib.rs-0315 */             // Determine how much was filled.
/* FP:lib.rs-0316 */             let mut chunks_borrow = self.chunks.borrow_mut();
/* FP:lib.rs-0317 */             if let Some(mut last_chunk) = chunks_borrow.pop() {
/* FP:lib.rs-0318 */                 // Drop the contents of the last chunk.
/* FP:lib.rs-0319 */                 self.clear_last_chunk(&mut last_chunk);
/* FP:lib.rs-0320 */                 // The last chunk will be dropped. Destroy all other chunks.
/* FP:lib.rs-0321 */                 for chunk in chunks_borrow.iter_mut() {
/* FP:lib.rs-0322 */                     chunk.destroy(chunk.entries);
/* FP:lib.rs-0323 */                 }
/* FP:lib.rs-0324 */             }
/* FP:lib.rs-0325 */             // Box handles deallocation of `last_chunk` and `self.chunks`.
/* FP:lib.rs-0326 */         }
/* FP:lib.rs-0327 */     }
/* FP:lib.rs-0328 */ }
/* FP:lib.rs-0329 */ 
/* FP:lib.rs-0330 */ unsafe impl<T: Send> Send for TypedArena<T> {}
/* FP:lib.rs-0331 */ 
/* FP:lib.rs-0332 */ #[inline(always)]
/* FP:lib.rs-0333 */ fn align_down(val: usize, align: usize) -> usize {
/* FP:lib.rs-0334 */     debug_assert!(align.is_power_of_two());
/* FP:lib.rs-0335 */     val & !(align - 1)
/* FP:lib.rs-0336 */ }
/* FP:lib.rs-0337 */ 
/* FP:lib.rs-0338 */ #[inline(always)]
/* FP:lib.rs-0339 */ fn align_up(val: usize, align: usize) -> usize {
/* FP:lib.rs-0340 */     debug_assert!(align.is_power_of_two());
/* FP:lib.rs-0341 */     (val + align - 1) & !(align - 1)
/* FP:lib.rs-0342 */ }
/* FP:lib.rs-0343 */ 
/* FP:lib.rs-0344 */ // Pointer alignment is common in compiler types, so keep `DroplessArena` aligned to them
/* FP:lib.rs-0345 */ // to optimize away alignment code.
/* FP:lib.rs-0346 */ const DROPLESS_ALIGNMENT: usize = align_of::<usize>();
/* FP:lib.rs-0347 */ 
/* FP:lib.rs-0348 */ /// An arena that can hold objects of multiple different types that impl `Copy`
/* FP:lib.rs-0349 */ /// and/or satisfy `!mem::needs_drop`.
/* FP:lib.rs-0350 */ pub struct DroplessArena {
/* FP:lib.rs-0351 */     /// A pointer to the start of the free space.
/* FP:lib.rs-0352 */     start: Cell<*mut u8>,
/* FP:lib.rs-0353 */ 
/* FP:lib.rs-0354 */     /// A pointer to the end of free space.
/* FP:lib.rs-0355 */     ///
/* FP:lib.rs-0356 */     /// The allocation proceeds downwards from the end of the chunk towards the
/* FP:lib.rs-0357 */     /// start. (This is slightly simpler and faster than allocating upwards,
/* FP:lib.rs-0358 */     /// see <https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html>.)
/* FP:lib.rs-0359 */     /// When this pointer crosses the start pointer, a new chunk is allocated.
/* FP:lib.rs-0360 */     ///
/* FP:lib.rs-0361 */     /// This is kept aligned to DROPLESS_ALIGNMENT.
/* FP:lib.rs-0362 */     end: Cell<*mut u8>,
/* FP:lib.rs-0363 */ 
/* FP:lib.rs-0364 */     /// A vector of arena chunks.
/* FP:lib.rs-0365 */     chunks: RefCell<Vec<ArenaChunk>>,
/* FP:lib.rs-0366 */ }
/* FP:lib.rs-0367 */ 
/* FP:lib.rs-0368 */ unsafe impl Send for DroplessArena {}
/* FP:lib.rs-0369 */ 
/* FP:lib.rs-0370 */ impl Default for DroplessArena {
/* FP:lib.rs-0371 */     #[inline]
/* FP:lib.rs-0372 */     fn default() -> DroplessArena {
/* FP:lib.rs-0373 */         DroplessArena {
/* FP:lib.rs-0374 */             // We set both `start` and `end` to 0 so that the first call to
/* FP:lib.rs-0375 */             // alloc() will trigger a grow().
/* FP:lib.rs-0376 */             start: Cell::new(ptr::null_mut()),
/* FP:lib.rs-0377 */             end: Cell::new(ptr::null_mut()),
/* FP:lib.rs-0378 */             chunks: Default::default(),
/* FP:lib.rs-0379 */         }
/* FP:lib.rs-0380 */     }
/* FP:lib.rs-0381 */ }
/* FP:lib.rs-0382 */ 
/* FP:lib.rs-0383 */ impl DroplessArena {
/* FP:lib.rs-0384 */     #[inline(never)]
/* FP:lib.rs-0385 */     #[cold]
/* FP:lib.rs-0386 */     fn grow(&self, layout: Layout) {
/* FP:lib.rs-0387 */         // Add some padding so we can align `self.end` while
/* FP:lib.rs-0388 */         // still fitting in a `layout` allocation.
/* FP:lib.rs-0389 */         let additional = layout.size() + cmp::max(DROPLESS_ALIGNMENT, layout.align()) - 1;
/* FP:lib.rs-0390 */ 
/* FP:lib.rs-0391 */         unsafe {
/* FP:lib.rs-0392 */             let mut chunks = self.chunks.borrow_mut();
/* FP:lib.rs-0393 */             let mut new_cap;
/* FP:lib.rs-0394 */             if let Some(last_chunk) = chunks.last_mut() {
/* FP:lib.rs-0395 */                 // There is no need to update `last_chunk.entries` because that
/* FP:lib.rs-0396 */                 // field isn't used by `DroplessArena`.
/* FP:lib.rs-0397 */ 
/* FP:lib.rs-0398 */                 // If the previous chunk's len is less than HUGE_PAGE
/* FP:lib.rs-0399 */                 // bytes, then this chunk will be least double the previous
/* FP:lib.rs-0400 */                 // chunk's size.
/* FP:lib.rs-0401 */                 new_cap = last_chunk.storage.len().min(HUGE_PAGE / 2);
/* FP:lib.rs-0402 */                 new_cap *= 2;
/* FP:lib.rs-0403 */             } else {
/* FP:lib.rs-0404 */                 new_cap = PAGE;
/* FP:lib.rs-0405 */             }
/* FP:lib.rs-0406 */             // Also ensure that this chunk can fit `additional`.
/* FP:lib.rs-0407 */             new_cap = cmp::max(additional, new_cap);
/* FP:lib.rs-0408 */ 
/* FP:lib.rs-0409 */             let mut chunk = ArenaChunk::new(align_up(new_cap, PAGE));
/* FP:lib.rs-0410 */             self.start.set(chunk.start());
/* FP:lib.rs-0411 */ 
/* FP:lib.rs-0412 */             // Align the end to DROPLESS_ALIGNMENT.
/* FP:lib.rs-0413 */             let end = align_down(chunk.end().addr(), DROPLESS_ALIGNMENT);
/* FP:lib.rs-0414 */ 
/* FP:lib.rs-0415 */             // Make sure we don't go past `start`. This should not happen since the allocation
/* FP:lib.rs-0416 */             // should be at least DROPLESS_ALIGNMENT - 1 bytes.
/* FP:lib.rs-0417 */             debug_assert!(chunk.start().addr() <= end);
/* FP:lib.rs-0418 */ 
/* FP:lib.rs-0419 */             self.end.set(chunk.end().with_addr(end));
/* FP:lib.rs-0420 */ 
/* FP:lib.rs-0421 */             chunks.push(chunk);
/* FP:lib.rs-0422 */         }
/* FP:lib.rs-0423 */     }
/* FP:lib.rs-0424 */ 
/* FP:lib.rs-0425 */     #[inline]
/* FP:lib.rs-0426 */     pub fn alloc_raw(&self, layout: Layout) -> *mut u8 {
/* FP:lib.rs-0427 */         assert!(layout.size() != 0);
/* FP:lib.rs-0428 */ 
/* FP:lib.rs-0429 */         // This loop executes once or twice: if allocation fails the first
/* FP:lib.rs-0430 */         // time, the `grow` ensures it will succeed the second time.
/* FP:lib.rs-0431 */         loop {
/* FP:lib.rs-0432 */             let start = self.start.get().addr();
/* FP:lib.rs-0433 */             let old_end = self.end.get();
/* FP:lib.rs-0434 */             let end = old_end.addr();
/* FP:lib.rs-0435 */ 
/* FP:lib.rs-0436 */             // Align allocated bytes so that `self.end` stays aligned to
/* FP:lib.rs-0437 */             // DROPLESS_ALIGNMENT.
/* FP:lib.rs-0438 */             let bytes = align_up(layout.size(), DROPLESS_ALIGNMENT);
/* FP:lib.rs-0439 */ 
/* FP:lib.rs-0440 */             // Tell LLVM that `end` is aligned to DROPLESS_ALIGNMENT.
/* FP:lib.rs-0441 */             unsafe { intrinsics::assume(end == align_down(end, DROPLESS_ALIGNMENT)) };
/* FP:lib.rs-0442 */ 
/* FP:lib.rs-0443 */             if let Some(sub) = end.checked_sub(bytes) {
/* FP:lib.rs-0444 */                 let new_end = align_down(sub, layout.align());
/* FP:lib.rs-0445 */                 if start <= new_end {
/* FP:lib.rs-0446 */                     let new_end = old_end.with_addr(new_end);
/* FP:lib.rs-0447 */                     // `new_end` is aligned to DROPLESS_ALIGNMENT as `align_down`
/* FP:lib.rs-0448 */                     // preserves alignment as both `end` and `bytes` are already
/* FP:lib.rs-0449 */                     // aligned to DROPLESS_ALIGNMENT.
/* FP:lib.rs-0450 */                     self.end.set(new_end);
/* FP:lib.rs-0451 */                     return new_end;
/* FP:lib.rs-0452 */                 }
/* FP:lib.rs-0453 */             }
/* FP:lib.rs-0454 */ 
/* FP:lib.rs-0455 */             // No free space left. Allocate a new chunk to satisfy the request.
/* FP:lib.rs-0456 */             // On failure the grow will panic or abort.
/* FP:lib.rs-0457 */             self.grow(layout);
/* FP:lib.rs-0458 */         }
/* FP:lib.rs-0459 */     }
/* FP:lib.rs-0460 */ 
/* FP:lib.rs-0461 */     #[inline]
/* FP:lib.rs-0462 */     pub fn alloc<T>(&self, object: T) -> &mut T {
/* FP:lib.rs-0463 */         assert!(!mem::needs_drop::<T>());
/* FP:lib.rs-0464 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0465 */ 
/* FP:lib.rs-0466 */         let mem = self.alloc_raw(Layout::new::<T>()) as *mut T;
/* FP:lib.rs-0467 */ 
/* FP:lib.rs-0468 */         unsafe {
/* FP:lib.rs-0469 */             // Write into uninitialized memory.
/* FP:lib.rs-0470 */             ptr::write(mem, object);
/* FP:lib.rs-0471 */             &mut *mem
/* FP:lib.rs-0472 */         }
/* FP:lib.rs-0473 */     }
/* FP:lib.rs-0474 */ 
/* FP:lib.rs-0475 */     /// Allocates a slice of objects that are copied into the `DroplessArena`, returning a mutable
/* FP:lib.rs-0476 */     /// reference to it. Will panic if passed a zero-sized type.
/* FP:lib.rs-0477 */     ///
/* FP:lib.rs-0478 */     /// Panics:
/* FP:lib.rs-0479 */     ///
/* FP:lib.rs-0480 */     ///  - Zero-sized types
/* FP:lib.rs-0481 */     ///  - Zero-length slices
/* FP:lib.rs-0482 */     #[inline]
/* FP:lib.rs-0483 */     pub fn alloc_slice<T>(&self, slice: &[T]) -> &mut [T]
/* FP:lib.rs-0484 */     where
/* FP:lib.rs-0485 */         T: Copy,
/* FP:lib.rs-0486 */     {
/* FP:lib.rs-0487 */         assert!(!mem::needs_drop::<T>());
/* FP:lib.rs-0488 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0489 */         assert!(!slice.is_empty());
/* FP:lib.rs-0490 */ 
/* FP:lib.rs-0491 */         let mem = self.alloc_raw(Layout::for_value::<[T]>(slice)) as *mut T;
/* FP:lib.rs-0492 */ 
/* FP:lib.rs-0493 */         unsafe {
/* FP:lib.rs-0494 */             mem.copy_from_nonoverlapping(slice.as_ptr(), slice.len());
/* FP:lib.rs-0495 */             slice::from_raw_parts_mut(mem, slice.len())
/* FP:lib.rs-0496 */         }
/* FP:lib.rs-0497 */     }
/* FP:lib.rs-0498 */ 
/* FP:lib.rs-0499 */     /// Used by `Lift` to check whether this slice is allocated
/* FP:lib.rs-0500 */     /// in this arena.
/* FP:lib.rs-0501 */     #[inline]
/* FP:lib.rs-0502 */     pub fn contains_slice<T>(&self, slice: &[T]) -> bool {
/* FP:lib.rs-0503 */         for chunk in self.chunks.borrow_mut().iter_mut() {
/* FP:lib.rs-0504 */             let ptr = slice.as_ptr().cast::<u8>().cast_mut();
/* FP:lib.rs-0505 */             if chunk.start() <= ptr && chunk.end() >= ptr {
/* FP:lib.rs-0506 */                 return true;
/* FP:lib.rs-0507 */             }
/* FP:lib.rs-0508 */         }
/* FP:lib.rs-0509 */         false
/* FP:lib.rs-0510 */     }
/* FP:lib.rs-0511 */ 
/* FP:lib.rs-0512 */     /// Allocates a string slice that is copied into the `DroplessArena`, returning a
/* FP:lib.rs-0513 */     /// reference to it. Will panic if passed an empty string.
/* FP:lib.rs-0514 */     ///
/* FP:lib.rs-0515 */     /// Panics:
/* FP:lib.rs-0516 */     ///
/* FP:lib.rs-0517 */     ///  - Zero-length string
/* FP:lib.rs-0518 */     #[inline]
/* FP:lib.rs-0519 */     pub fn alloc_str(&self, string: &str) -> &str {
/* FP:lib.rs-0520 */         let slice = self.alloc_slice(string.as_bytes());
/* FP:lib.rs-0521 */ 
/* FP:lib.rs-0522 */         // SAFETY: the result has a copy of the same valid UTF-8 bytes.
/* FP:lib.rs-0523 */         unsafe { std::str::from_utf8_unchecked(slice) }
/* FP:lib.rs-0524 */     }
/* FP:lib.rs-0525 */ 
/* FP:lib.rs-0526 */     /// # Safety
/* FP:lib.rs-0527 */     ///
/* FP:lib.rs-0528 */     /// The caller must ensure that `mem` is valid for writes up to `size_of::<T>() * len`, and that
/* FP:lib.rs-0529 */     /// that memory stays allocated and not shared for the lifetime of `self`. This must hold even
/* FP:lib.rs-0530 */     /// if `iter.next()` allocates onto `self`.
/* FP:lib.rs-0531 */     #[inline]
/* FP:lib.rs-0532 */     unsafe fn write_from_iter<T, I: Iterator<Item = T>>(
/* FP:lib.rs-0533 */         &self,
/* FP:lib.rs-0534 */         mut iter: I,
/* FP:lib.rs-0535 */         len: usize,
/* FP:lib.rs-0536 */         mem: *mut T,
/* FP:lib.rs-0537 */     ) -> &mut [T] {
/* FP:lib.rs-0538 */         let mut i = 0;
/* FP:lib.rs-0539 */         // Use a manual loop since LLVM manages to optimize it better for
/* FP:lib.rs-0540 */         // slice iterators
/* FP:lib.rs-0541 */         loop {
/* FP:lib.rs-0542 */             // SAFETY: The caller must ensure that `mem` is valid for writes up to
/* FP:lib.rs-0543 */             // `size_of::<T>() * len`.
/* FP:lib.rs-0544 */             unsafe {
/* FP:lib.rs-0545 */                 match iter.next() {
/* FP:lib.rs-0546 */                     Some(value) if i < len => mem.add(i).write(value),
/* FP:lib.rs-0547 */                     Some(_) | None => {
/* FP:lib.rs-0548 */                         // We only return as many items as the iterator gave us, even
/* FP:lib.rs-0549 */                         // though it was supposed to give us `len`
/* FP:lib.rs-0550 */                         return slice::from_raw_parts_mut(mem, i);
/* FP:lib.rs-0551 */                     }
/* FP:lib.rs-0552 */                 }
/* FP:lib.rs-0553 */             }
/* FP:lib.rs-0554 */             i += 1;
/* FP:lib.rs-0555 */         }
/* FP:lib.rs-0556 */     }
/* FP:lib.rs-0557 */ 
/* FP:lib.rs-0558 */     #[inline]
/* FP:lib.rs-0559 */     pub fn alloc_from_iter<T, I: IntoIterator<Item = T>>(&self, iter: I) -> &mut [T] {
/* FP:lib.rs-0560 */         // Warning: this function is reentrant: `iter` could hold a reference to `&self` and
/* FP:lib.rs-0561 */         // allocate additional elements while we're iterating.
/* FP:lib.rs-0562 */         let iter = iter.into_iter();
/* FP:lib.rs-0563 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0564 */         assert!(!mem::needs_drop::<T>());
/* FP:lib.rs-0565 */ 
/* FP:lib.rs-0566 */         let size_hint = iter.size_hint();
/* FP:lib.rs-0567 */ 
/* FP:lib.rs-0568 */         match size_hint {
/* FP:lib.rs-0569 */             (min, Some(max)) if min == max => {
/* FP:lib.rs-0570 */                 // We know the exact number of elements the iterator expects to produce here.
/* FP:lib.rs-0571 */                 let len = min;
/* FP:lib.rs-0572 */ 
/* FP:lib.rs-0573 */                 if len == 0 {
/* FP:lib.rs-0574 */                     return &mut [];
/* FP:lib.rs-0575 */                 }
/* FP:lib.rs-0576 */ 
/* FP:lib.rs-0577 */                 let mem = self.alloc_raw(Layout::array::<T>(len).unwrap()) as *mut T;
/* FP:lib.rs-0578 */                 // SAFETY: `write_from_iter` doesn't touch `self`. It only touches the slice we just
/* FP:lib.rs-0579 */                 // reserved. If the iterator panics or doesn't output `len` elements, this will
/* FP:lib.rs-0580 */                 // leave some unallocated slots in the arena, which is fine because we do not call
/* FP:lib.rs-0581 */                 // `drop`.
/* FP:lib.rs-0582 */                 unsafe { self.write_from_iter(iter, len, mem) }
/* FP:lib.rs-0583 */             }
/* FP:lib.rs-0584 */             (_, _) => outline(move || self.try_alloc_from_iter(iter.map(Ok::<T, !>)).into_ok()),
/* FP:lib.rs-0585 */         }
/* FP:lib.rs-0586 */     }
/* FP:lib.rs-0587 */ 
/* FP:lib.rs-0588 */     #[inline]
/* FP:lib.rs-0589 */     pub fn try_alloc_from_iter<T, E>(
/* FP:lib.rs-0590 */         &self,
/* FP:lib.rs-0591 */         iter: impl IntoIterator<Item = Result<T, E>>,
/* FP:lib.rs-0592 */     ) -> Result<&mut [T], E> {
/* FP:lib.rs-0593 */         // Despite the similarlty with `alloc_from_iter`, we cannot reuse their fast case, as we
/* FP:lib.rs-0594 */         // cannot know the minimum length of the iterator in this case.
/* FP:lib.rs-0595 */         assert!(size_of::<T>() != 0);
/* FP:lib.rs-0596 */ 
/* FP:lib.rs-0597 */         // Takes care of reentrancy.
/* FP:lib.rs-0598 */         let vec: Result<SmallVec<[T; 8]>, E> = iter.into_iter().collect();
/* FP:lib.rs-0599 */         let mut vec = vec?;
/* FP:lib.rs-0600 */         if vec.is_empty() {
/* FP:lib.rs-0601 */             return Ok(&mut []);
/* FP:lib.rs-0602 */         }
/* FP:lib.rs-0603 */         // Move the content to the arena by copying and then forgetting it.
/* FP:lib.rs-0604 */         let len = vec.len();
/* FP:lib.rs-0605 */         Ok(unsafe {
/* FP:lib.rs-0606 */             let start_ptr = self.alloc_raw(Layout::for_value::<[T]>(vec.as_slice())) as *mut T;
/* FP:lib.rs-0607 */             vec.as_ptr().copy_to_nonoverlapping(start_ptr, len);
/* FP:lib.rs-0608 */             vec.set_len(0);
/* FP:lib.rs-0609 */             slice::from_raw_parts_mut(start_ptr, len)
/* FP:lib.rs-0610 */         })
/* FP:lib.rs-0611 */     }
/* FP:lib.rs-0612 */ }
/* FP:lib.rs-0613 */ 
/* FP:lib.rs-0614 */ /// Declare an `Arena` containing one dropless arena and many typed arenas (the
/* FP:lib.rs-0615 */ /// types of the typed arenas are specified by the arguments).
/* FP:lib.rs-0616 */ ///
/* FP:lib.rs-0617 */ /// There are three cases of interest.
/* FP:lib.rs-0618 */ /// - Types that are `Copy`: these need not be specified in the arguments. They
/* FP:lib.rs-0619 */ ///   will use the `DroplessArena`.
/* FP:lib.rs-0620 */ /// - Types that are `!Copy` and `!Drop`: these must be specified in the
/* FP:lib.rs-0621 */ ///   arguments. An empty `TypedArena` will be created for each one, but the
/* FP:lib.rs-0622 */ ///   `DroplessArena` will always be used and the `TypedArena` will stay empty.
/* FP:lib.rs-0623 */ ///   This is odd but harmless, because an empty arena allocates no memory.
/* FP:lib.rs-0624 */ /// - Types that are `!Copy` and `Drop`: these must be specified in the
/* FP:lib.rs-0625 */ ///   arguments. The `TypedArena` will be used for them.
/* FP:lib.rs-0626 */ ///
/* FP:lib.rs-0627 */ #[rustc_macro_transparency = "semitransparent"]
/* FP:lib.rs-0628 */ pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
/* FP:lib.rs-0629 */     #[derive(Default)]
/* FP:lib.rs-0630 */     pub struct Arena<'tcx> {
/* FP:lib.rs-0631 */         pub dropless: $crate::DroplessArena,
/* FP:lib.rs-0632 */         $($name: $crate::TypedArena<$ty>,)*
/* FP:lib.rs-0633 */     }
/* FP:lib.rs-0634 */ 
/* FP:lib.rs-0635 */     pub trait ArenaAllocatable<'tcx, C = rustc_arena::IsNotCopy>: Sized {
/* FP:lib.rs-0636 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0637 */         fn allocate_on(self, arena: &'tcx Arena<'tcx>) -> &'tcx mut Self;
/* FP:lib.rs-0638 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0639 */         fn allocate_from_iter(
/* FP:lib.rs-0640 */             arena: &'tcx Arena<'tcx>,
/* FP:lib.rs-0641 */             iter: impl ::std::iter::IntoIterator<Item = Self>,
/* FP:lib.rs-0642 */         ) -> &'tcx mut [Self];
/* FP:lib.rs-0643 */     }
/* FP:lib.rs-0644 */ 
/* FP:lib.rs-0645 */     // Any type that impls `Copy` can be arena-allocated in the `DroplessArena`.
/* FP:lib.rs-0646 */     impl<'tcx, T: Copy> ArenaAllocatable<'tcx, rustc_arena::IsCopy> for T {
/* FP:lib.rs-0647 */         #[inline]
/* FP:lib.rs-0648 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0649 */         fn allocate_on(self, arena: &'tcx Arena<'tcx>) -> &'tcx mut Self {
/* FP:lib.rs-0650 */             arena.dropless.alloc(self)
/* FP:lib.rs-0651 */         }
/* FP:lib.rs-0652 */         #[inline]
/* FP:lib.rs-0653 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0654 */         fn allocate_from_iter(
/* FP:lib.rs-0655 */             arena: &'tcx Arena<'tcx>,
/* FP:lib.rs-0656 */             iter: impl ::std::iter::IntoIterator<Item = Self>,
/* FP:lib.rs-0657 */         ) -> &'tcx mut [Self] {
/* FP:lib.rs-0658 */             arena.dropless.alloc_from_iter(iter)
/* FP:lib.rs-0659 */         }
/* FP:lib.rs-0660 */     }
/* FP:lib.rs-0661 */     $(
/* FP:lib.rs-0662 */         impl<'tcx> ArenaAllocatable<'tcx, rustc_arena::IsNotCopy> for $ty {
/* FP:lib.rs-0663 */             #[inline]
/* FP:lib.rs-0664 */             fn allocate_on(self, arena: &'tcx Arena<'tcx>) -> &'tcx mut Self {
/* FP:lib.rs-0665 */                 if !::std::mem::needs_drop::<Self>() {
/* FP:lib.rs-0666 */                     arena.dropless.alloc(self)
/* FP:lib.rs-0667 */                 } else {
/* FP:lib.rs-0668 */                     arena.$name.alloc(self)
/* FP:lib.rs-0669 */                 }
/* FP:lib.rs-0670 */             }
/* FP:lib.rs-0671 */ 
/* FP:lib.rs-0672 */             #[inline]
/* FP:lib.rs-0673 */             #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0674 */             fn allocate_from_iter(
/* FP:lib.rs-0675 */                 arena: &'tcx Arena<'tcx>,
/* FP:lib.rs-0676 */                 iter: impl ::std::iter::IntoIterator<Item = Self>,
/* FP:lib.rs-0677 */             ) -> &'tcx mut [Self] {
/* FP:lib.rs-0678 */                 if !::std::mem::needs_drop::<Self>() {
/* FP:lib.rs-0679 */                     arena.dropless.alloc_from_iter(iter)
/* FP:lib.rs-0680 */                 } else {
/* FP:lib.rs-0681 */                     arena.$name.alloc_from_iter(iter)
/* FP:lib.rs-0682 */                 }
/* FP:lib.rs-0683 */             }
/* FP:lib.rs-0684 */         }
/* FP:lib.rs-0685 */     )*
/* FP:lib.rs-0686 */ 
/* FP:lib.rs-0687 */     impl<'tcx> Arena<'tcx> {
/* FP:lib.rs-0688 */         #[inline]
/* FP:lib.rs-0689 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0690 */         pub fn alloc<T: ArenaAllocatable<'tcx, C>, C>(&'tcx self, value: T) -> &mut T {
/* FP:lib.rs-0691 */             value.allocate_on(self)
/* FP:lib.rs-0692 */         }
/* FP:lib.rs-0693 */ 
/* FP:lib.rs-0694 */         // Any type that impls `Copy` can have slices be arena-allocated in the `DroplessArena`.
/* FP:lib.rs-0695 */         #[inline]
/* FP:lib.rs-0696 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0697 */         pub fn alloc_slice<T: ::std::marker::Copy>(&self, value: &[T]) -> &mut [T] {
/* FP:lib.rs-0698 */             if value.is_empty() {
/* FP:lib.rs-0699 */                 return &mut [];
/* FP:lib.rs-0700 */             }
/* FP:lib.rs-0701 */             self.dropless.alloc_slice(value)
/* FP:lib.rs-0702 */         }
/* FP:lib.rs-0703 */ 
/* FP:lib.rs-0704 */         #[inline]
/* FP:lib.rs-0705 */         pub fn alloc_str(&self, string: &str) -> &str {
/* FP:lib.rs-0706 */             if string.is_empty() {
/* FP:lib.rs-0707 */                 return "";
/* FP:lib.rs-0708 */             }
/* FP:lib.rs-0709 */             self.dropless.alloc_str(string)
/* FP:lib.rs-0710 */         }
/* FP:lib.rs-0711 */ 
/* FP:lib.rs-0712 */         #[allow(clippy::mut_from_ref)]
/* FP:lib.rs-0713 */         pub fn alloc_from_iter<T: ArenaAllocatable<'tcx, C>, C>(
/* FP:lib.rs-0714 */             &'tcx self,
/* FP:lib.rs-0715 */             iter: impl ::std::iter::IntoIterator<Item = T>,
/* FP:lib.rs-0716 */         ) -> &mut [T] {
/* FP:lib.rs-0717 */             T::allocate_from_iter(self, iter)
/* FP:lib.rs-0718 */         }
/* FP:lib.rs-0719 */     }
/* FP:lib.rs-0720 */ }
/* FP:lib.rs-0721 */ 
/* FP:lib.rs-0722 */ // Marker types that let us give different behaviour for arenas allocating
/* FP:lib.rs-0723 */ // `Copy` types vs `!Copy` types.
/* FP:lib.rs-0724 */ pub struct IsCopy;
/* FP:lib.rs-0725 */ pub struct IsNotCopy;
/* FP:lib.rs-0726 */ 
/* FP:lib.rs-0727 */ #[cfg(test)]