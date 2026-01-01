/* FP:vec.rs-0001 */ use std::borrow::{Borrow, BorrowMut};
/* FP:vec.rs-0002 */ use std::hash::Hash;
/* FP:vec.rs-0003 */ use std::marker::PhantomData;
/* FP:vec.rs-0004 */ use std::ops::{Deref, DerefMut, RangeBounds};
/* FP:vec.rs-0005 */ use std::{fmt, slice, vec};
/* FP:vec.rs-0006 */ 
/* FP:vec.rs-0007 */ #[cfg(feature = "nightly")]
/* FP:vec.rs-0008 */ use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* FP:vec.rs-0009 */ 
/* FP:vec.rs-0010 */ use crate::{Idx, IndexSlice};
/* FP:vec.rs-0011 */ 
/* FP:vec.rs-0012 */ /// An owned contiguous collection of `T`s, indexed by `I` rather than by `usize`.
/* FP:vec.rs-0013 */ ///
/* FP:vec.rs-0014 */ /// ## Why use this instead of a `Vec`?
/* FP:vec.rs-0015 */ ///
/* FP:vec.rs-0016 */ /// An `IndexVec` allows element access only via a specific associated index type, meaning that
/* FP:vec.rs-0017 */ /// trying to use the wrong index type (possibly accessing an invalid element) will fail at
/* FP:vec.rs-0018 */ /// compile time.
/* FP:vec.rs-0019 */ ///
/* FP:vec.rs-0020 */ /// It also documents what the index is indexing: in a `HashMap<usize, Something>` it's not
/* FP:vec.rs-0021 */ /// immediately clear what the `usize` means, while a `HashMap<FieldIdx, Something>` makes it obvious.
/* FP:vec.rs-0022 */ ///
/* FP:vec.rs-0023 */ /// ```compile_fail
/* FP:vec.rs-0024 */ /// use crate::rustc_index::{Idx, IndexVec};
/* FP:vec.rs-0025 */ ///
/* FP:vec.rs-0026 */ /// fn f<I1: Idx, I2: Idx>(vec1: IndexVec<I1, u8>, idx1: I1, idx2: I2) {
/* FP:vec.rs-0027 */ ///   &vec1[idx1]; // Ok
/* FP:vec.rs-0028 */ ///   &vec1[idx2]; // Compile error!
/* FP:vec.rs-0029 */ /// }
/* FP:vec.rs-0030 */ /// ```
/* FP:vec.rs-0031 */ ///
/* FP:vec.rs-0032 */ /// While it's possible to use `u32` or `usize` directly for `I`,
/* FP:vec.rs-0033 */ /// you almost certainly want to use a [`newtype_index!`]-generated type instead.
/* FP:vec.rs-0034 */ ///
/* FP:vec.rs-0035 */ /// This allows to index the IndexVec with the new index type.
/* FP:vec.rs-0036 */ ///
/* FP:vec.rs-0037 */ /// [`newtype_index!`]: ../macro.newtype_index.html
/* FP:vec.rs-0038 */ #[derive(Clone, PartialEq, Eq, Hash)]
/* FP:vec.rs-0039 */ #[repr(transparent)]
/* FP:vec.rs-0040 */ pub struct IndexVec<I: Idx, T> {
/* FP:vec.rs-0041 */     pub raw: Vec<T>,
/* FP:vec.rs-0042 */     _marker: PhantomData<fn(&I)>,
/* FP:vec.rs-0043 */ }
/* FP:vec.rs-0044 */ 
/* FP:vec.rs-0045 */ impl<I: Idx, T> IndexVec<I, T> {
/* FP:vec.rs-0046 */     /// Constructs a new, empty `IndexVec<I, T>`.
/* FP:vec.rs-0047 */     #[inline]
/* FP:vec.rs-0048 */     pub const fn new() -> Self {
/* FP:vec.rs-0049 */         IndexVec::from_raw(Vec::new())
/* FP:vec.rs-0050 */     }
/* FP:vec.rs-0051 */ 
/* FP:vec.rs-0052 */     /// Constructs a new `IndexVec<I, T>` from a `Vec<T>`.
/* FP:vec.rs-0053 */     #[inline]
/* FP:vec.rs-0054 */     pub const fn from_raw(raw: Vec<T>) -> Self {
/* FP:vec.rs-0055 */         IndexVec { raw, _marker: PhantomData }
/* FP:vec.rs-0056 */     }
/* FP:vec.rs-0057 */ 
/* FP:vec.rs-0058 */     #[inline]
/* FP:vec.rs-0059 */     pub fn with_capacity(capacity: usize) -> Self {
/* FP:vec.rs-0060 */         IndexVec::from_raw(Vec::with_capacity(capacity))
/* FP:vec.rs-0061 */     }
/* FP:vec.rs-0062 */ 
/* FP:vec.rs-0063 */     /// Creates a new vector with a copy of `elem` for each index in `universe`.
/* FP:vec.rs-0064 */     ///
/* FP:vec.rs-0065 */     /// Thus `IndexVec::from_elem(elem, &universe)` is equivalent to
/* FP:vec.rs-0066 */     /// `IndexVec::<I, _>::from_elem_n(elem, universe.len())`. That can help
/* FP:vec.rs-0067 */     /// type inference as it ensures that the resulting vector uses the same
/* FP:vec.rs-0068 */     /// index type as `universe`, rather than something potentially surprising.
/* FP:vec.rs-0069 */     ///
/* FP:vec.rs-0070 */     /// For example, if you want to store data for each local in a MIR body,
/* FP:vec.rs-0071 */     /// using `let mut uses = IndexVec::from_elem(vec![], &body.local_decls);`
/* FP:vec.rs-0072 */     /// ensures that `uses` is an `IndexVec<Local, _>`, and thus can give
/* FP:vec.rs-0073 */     /// better error messages later if one accidentally mismatches indices.
/* FP:vec.rs-0074 */     #[inline]
/* FP:vec.rs-0075 */     pub fn from_elem<S>(elem: T, universe: &IndexSlice<I, S>) -> Self
/* FP:vec.rs-0076 */     where
/* FP:vec.rs-0077 */         T: Clone,
/* FP:vec.rs-0078 */     {
/* FP:vec.rs-0079 */         IndexVec::from_raw(vec![elem; universe.len()])
/* FP:vec.rs-0080 */     }
/* FP:vec.rs-0081 */ 
/* FP:vec.rs-0082 */     /// Creates a new IndexVec with n copies of the `elem`.
/* FP:vec.rs-0083 */     #[inline]
/* FP:vec.rs-0084 */     pub fn from_elem_n(elem: T, n: usize) -> Self
/* FP:vec.rs-0085 */     where
/* FP:vec.rs-0086 */         T: Clone,
/* FP:vec.rs-0087 */     {
/* FP:vec.rs-0088 */         IndexVec::from_raw(vec![elem; n])
/* FP:vec.rs-0089 */     }
/* FP:vec.rs-0090 */ 
/* FP:vec.rs-0091 */     /// Create an `IndexVec` with `n` elements, where the value of each
/* FP:vec.rs-0092 */     /// element is the result of `func(i)`. (The underlying vector will
/* FP:vec.rs-0093 */     /// be allocated only once, with a capacity of at least `n`.)
/* FP:vec.rs-0094 */     #[inline]
/* FP:vec.rs-0095 */     pub fn from_fn_n(func: impl FnMut(I) -> T, n: usize) -> Self {
/* FP:vec.rs-0096 */         // Allow the optimizer to elide the bounds checking when creating each index.
/* FP:vec.rs-0097 */         let _ = I::new(n);
/* FP:vec.rs-0098 */         IndexVec::from_raw((0..n).map(I::new).map(func).collect())
/* FP:vec.rs-0099 */     }
/* FP:vec.rs-0100 */ 
/* FP:vec.rs-0101 */     #[inline]
/* FP:vec.rs-0102 */     pub fn as_slice(&self) -> &IndexSlice<I, T> {
/* FP:vec.rs-0103 */         IndexSlice::from_raw(&self.raw)
/* FP:vec.rs-0104 */     }
/* FP:vec.rs-0105 */ 
/* FP:vec.rs-0106 */     #[inline]
/* FP:vec.rs-0107 */     pub fn as_mut_slice(&mut self) -> &mut IndexSlice<I, T> {
/* FP:vec.rs-0108 */         IndexSlice::from_raw_mut(&mut self.raw)
/* FP:vec.rs-0109 */     }
/* FP:vec.rs-0110 */ 
/* FP:vec.rs-0111 */     /// Pushes an element to the array returning the index where it was pushed to.
/* FP:vec.rs-0112 */     #[inline]
/* FP:vec.rs-0113 */     pub fn push(&mut self, d: T) -> I {
/* FP:vec.rs-0114 */         let idx = self.next_index();
/* FP:vec.rs-0115 */         self.raw.push(d);
/* FP:vec.rs-0116 */         idx
/* FP:vec.rs-0117 */     }
/* FP:vec.rs-0118 */ 
/* FP:vec.rs-0119 */     #[inline]
/* FP:vec.rs-0120 */     pub fn pop(&mut self) -> Option<T> {
/* FP:vec.rs-0121 */         self.raw.pop()
/* FP:vec.rs-0122 */     }
/* FP:vec.rs-0123 */ 
/* FP:vec.rs-0124 */     #[inline]
/* FP:vec.rs-0125 */     pub fn into_iter(self) -> vec::IntoIter<T> {
/* FP:vec.rs-0126 */         self.raw.into_iter()
/* FP:vec.rs-0127 */     }
/* FP:vec.rs-0128 */ 
/* FP:vec.rs-0129 */     #[inline]
/* FP:vec.rs-0130 */     pub fn into_iter_enumerated(
/* FP:vec.rs-0131 */         self,
/* FP:vec.rs-0132 */     ) -> impl DoubleEndedIterator<Item = (I, T)> + ExactSizeIterator {
/* FP:vec.rs-0133 */         // Allow the optimizer to elide the bounds checking when creating each index.
/* FP:vec.rs-0134 */         let _ = I::new(self.len());
/* FP:vec.rs-0135 */         self.raw.into_iter().enumerate().map(|(n, t)| (I::new(n), t))
/* FP:vec.rs-0136 */     }
/* FP:vec.rs-0137 */ 
/* FP:vec.rs-0138 */     #[inline]
/* FP:vec.rs-0139 */     pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> impl Iterator<Item = T> {
/* FP:vec.rs-0140 */         self.raw.drain(range)
/* FP:vec.rs-0141 */     }
/* FP:vec.rs-0142 */ 
/* FP:vec.rs-0143 */     #[inline]
/* FP:vec.rs-0144 */     pub fn drain_enumerated<R: RangeBounds<usize>>(
/* FP:vec.rs-0145 */         &mut self,
/* FP:vec.rs-0146 */         range: R,
/* FP:vec.rs-0147 */     ) -> impl Iterator<Item = (I, T)> {
/* FP:vec.rs-0148 */         let begin = match range.start_bound() {
/* FP:vec.rs-0149 */             std::ops::Bound::Included(i) => *i,
/* FP:vec.rs-0150 */             std::ops::Bound::Excluded(i) => i.checked_add(1).unwrap(),
/* FP:vec.rs-0151 */             std::ops::Bound::Unbounded => 0,
/* FP:vec.rs-0152 */         };
/* FP:vec.rs-0153 */         self.raw.drain(range).enumerate().map(move |(n, t)| (I::new(begin + n), t))
/* FP:vec.rs-0154 */     }
/* FP:vec.rs-0155 */ 
/* FP:vec.rs-0156 */     #[inline]
/* FP:vec.rs-0157 */     pub fn shrink_to_fit(&mut self) {
/* FP:vec.rs-0158 */         self.raw.shrink_to_fit()
/* FP:vec.rs-0159 */     }
/* FP:vec.rs-0160 */ 
/* FP:vec.rs-0161 */     #[inline]
/* FP:vec.rs-0162 */     pub fn truncate(&mut self, a: usize) {
/* FP:vec.rs-0163 */         self.raw.truncate(a)
/* FP:vec.rs-0164 */     }
/* FP:vec.rs-0165 */ 
/* FP:vec.rs-0166 */     /// Grows the index vector so that it contains an entry for
/* FP:vec.rs-0167 */     /// `elem`; if that is already true, then has no
/* FP:vec.rs-0168 */     /// effect. Otherwise, inserts new values as needed by invoking
/* FP:vec.rs-0169 */     /// `fill_value`.
/* FP:vec.rs-0170 */     ///
/* FP:vec.rs-0171 */     /// Returns a reference to the `elem` entry.
/* FP:vec.rs-0172 */     #[inline]
/* FP:vec.rs-0173 */     pub fn ensure_contains_elem(&mut self, elem: I, fill_value: impl FnMut() -> T) -> &mut T {
/* FP:vec.rs-0174 */         let min_new_len = elem.index() + 1;
/* FP:vec.rs-0175 */         if self.len() < min_new_len {
/* FP:vec.rs-0176 */             self.raw.resize_with(min_new_len, fill_value);
/* FP:vec.rs-0177 */         }
/* FP:vec.rs-0178 */ 
/* FP:vec.rs-0179 */         &mut self[elem]
/* FP:vec.rs-0180 */     }
/* FP:vec.rs-0181 */ 
/* FP:vec.rs-0182 */     #[inline]
/* FP:vec.rs-0183 */     pub fn resize(&mut self, new_len: usize, value: T)
/* FP:vec.rs-0184 */     where
/* FP:vec.rs-0185 */         T: Clone,
/* FP:vec.rs-0186 */     {
/* FP:vec.rs-0187 */         self.raw.resize(new_len, value)
/* FP:vec.rs-0188 */     }
/* FP:vec.rs-0189 */ 
/* FP:vec.rs-0190 */     #[inline]
/* FP:vec.rs-0191 */     pub fn resize_to_elem(&mut self, elem: I, fill_value: impl FnMut() -> T) {
/* FP:vec.rs-0192 */         let min_new_len = elem.index() + 1;
/* FP:vec.rs-0193 */         self.raw.resize_with(min_new_len, fill_value);
/* FP:vec.rs-0194 */     }
/* FP:vec.rs-0195 */ 
/* FP:vec.rs-0196 */     #[inline]
/* FP:vec.rs-0197 */     pub fn append(&mut self, other: &mut Self) {
/* FP:vec.rs-0198 */         self.raw.append(&mut other.raw);
/* FP:vec.rs-0199 */     }
/* FP:vec.rs-0200 */ }
/* FP:vec.rs-0201 */ 
/* FP:vec.rs-0202 */ /// `IndexVec` is often used as a map, so it provides some map-like APIs.
/* FP:vec.rs-0203 */ impl<I: Idx, T> IndexVec<I, Option<T>> {
/* FP:vec.rs-0204 */     #[inline]
/* FP:vec.rs-0205 */     pub fn insert(&mut self, index: I, value: T) -> Option<T> {
/* FP:vec.rs-0206 */         self.ensure_contains_elem(index, || None).replace(value)
/* FP:vec.rs-0207 */     }
/* FP:vec.rs-0208 */ 
/* FP:vec.rs-0209 */     #[inline]
/* FP:vec.rs-0210 */     pub fn get_or_insert_with(&mut self, index: I, value: impl FnOnce() -> T) -> &mut T {
/* FP:vec.rs-0211 */         self.ensure_contains_elem(index, || None).get_or_insert_with(value)
/* FP:vec.rs-0212 */     }
/* FP:vec.rs-0213 */ 
/* FP:vec.rs-0214 */     #[inline]
/* FP:vec.rs-0215 */     pub fn remove(&mut self, index: I) -> Option<T> {
/* FP:vec.rs-0216 */         self.get_mut(index)?.take()
/* FP:vec.rs-0217 */     }
/* FP:vec.rs-0218 */ 
/* FP:vec.rs-0219 */     #[inline]
/* FP:vec.rs-0220 */     pub fn contains(&self, index: I) -> bool {
/* FP:vec.rs-0221 */         self.get(index).and_then(Option::as_ref).is_some()
/* FP:vec.rs-0222 */     }
/* FP:vec.rs-0223 */ }
/* FP:vec.rs-0224 */ 
/* FP:vec.rs-0225 */ impl<I: Idx, T: fmt::Debug> fmt::Debug for IndexVec<I, T> {
/* FP:vec.rs-0226 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:vec.rs-0227 */         fmt::Debug::fmt(&self.raw, fmt)
/* FP:vec.rs-0228 */     }
/* FP:vec.rs-0229 */ }
/* FP:vec.rs-0230 */ 
/* FP:vec.rs-0231 */ impl<I: Idx, T> Deref for IndexVec<I, T> {
/* FP:vec.rs-0232 */     type Target = IndexSlice<I, T>;
/* FP:vec.rs-0233 */ 
/* FP:vec.rs-0234 */     #[inline]
/* FP:vec.rs-0235 */     fn deref(&self) -> &Self::Target {
/* FP:vec.rs-0236 */         self.as_slice()
/* FP:vec.rs-0237 */     }
/* FP:vec.rs-0238 */ }
/* FP:vec.rs-0239 */ 
/* FP:vec.rs-0240 */ impl<I: Idx, T> DerefMut for IndexVec<I, T> {
/* FP:vec.rs-0241 */     #[inline]
/* FP:vec.rs-0242 */     fn deref_mut(&mut self) -> &mut Self::Target {
/* FP:vec.rs-0243 */         self.as_mut_slice()
/* FP:vec.rs-0244 */     }
/* FP:vec.rs-0245 */ }
/* FP:vec.rs-0246 */ 
/* FP:vec.rs-0247 */ impl<I: Idx, T> Borrow<IndexSlice<I, T>> for IndexVec<I, T> {
/* FP:vec.rs-0248 */     fn borrow(&self) -> &IndexSlice<I, T> {
/* FP:vec.rs-0249 */         self
/* FP:vec.rs-0250 */     }
/* FP:vec.rs-0251 */ }
/* FP:vec.rs-0252 */ 
/* FP:vec.rs-0253 */ impl<I: Idx, T> BorrowMut<IndexSlice<I, T>> for IndexVec<I, T> {
/* FP:vec.rs-0254 */     fn borrow_mut(&mut self) -> &mut IndexSlice<I, T> {
/* FP:vec.rs-0255 */         self
/* FP:vec.rs-0256 */     }
/* FP:vec.rs-0257 */ }
/* FP:vec.rs-0258 */ 
/* FP:vec.rs-0259 */ impl<I: Idx, T> Extend<T> for IndexVec<I, T> {
/* FP:vec.rs-0260 */     #[inline]
/* FP:vec.rs-0261 */     fn extend<J: IntoIterator<Item = T>>(&mut self, iter: J) {
/* FP:vec.rs-0262 */         self.raw.extend(iter);
/* FP:vec.rs-0263 */     }
/* FP:vec.rs-0264 */ 
/* FP:vec.rs-0265 */     #[inline]
/* FP:vec.rs-0266 */     #[cfg(feature = "nightly")]
/* FP:vec.rs-0267 */     fn extend_one(&mut self, item: T) {
/* FP:vec.rs-0268 */         self.raw.push(item);
/* FP:vec.rs-0269 */     }
/* FP:vec.rs-0270 */ 
/* FP:vec.rs-0271 */     #[inline]
/* FP:vec.rs-0272 */     #[cfg(feature = "nightly")]
/* FP:vec.rs-0273 */     fn extend_reserve(&mut self, additional: usize) {
/* FP:vec.rs-0274 */         self.raw.reserve(additional);
/* FP:vec.rs-0275 */     }
/* FP:vec.rs-0276 */ }
/* FP:vec.rs-0277 */ 
/* FP:vec.rs-0278 */ impl<I: Idx, T> FromIterator<T> for IndexVec<I, T> {
/* FP:vec.rs-0279 */     #[inline]
/* FP:vec.rs-0280 */     fn from_iter<J>(iter: J) -> Self
/* FP:vec.rs-0281 */     where
/* FP:vec.rs-0282 */         J: IntoIterator<Item = T>,
/* FP:vec.rs-0283 */     {
/* FP:vec.rs-0284 */         IndexVec::from_raw(Vec::from_iter(iter))
/* FP:vec.rs-0285 */     }
/* FP:vec.rs-0286 */ }
/* FP:vec.rs-0287 */ 
/* FP:vec.rs-0288 */ impl<I: Idx, T> IntoIterator for IndexVec<I, T> {
/* FP:vec.rs-0289 */     type Item = T;
/* FP:vec.rs-0290 */     type IntoIter = vec::IntoIter<T>;
/* FP:vec.rs-0291 */ 
/* FP:vec.rs-0292 */     #[inline]
/* FP:vec.rs-0293 */     fn into_iter(self) -> vec::IntoIter<T> {
/* FP:vec.rs-0294 */         self.raw.into_iter()
/* FP:vec.rs-0295 */     }
/* FP:vec.rs-0296 */ }
/* FP:vec.rs-0297 */ 
/* FP:vec.rs-0298 */ impl<'a, I: Idx, T> IntoIterator for &'a IndexVec<I, T> {
/* FP:vec.rs-0299 */     type Item = &'a T;
/* FP:vec.rs-0300 */     type IntoIter = slice::Iter<'a, T>;
/* FP:vec.rs-0301 */ 
/* FP:vec.rs-0302 */     #[inline]
/* FP:vec.rs-0303 */     fn into_iter(self) -> slice::Iter<'a, T> {
/* FP:vec.rs-0304 */         self.iter()
/* FP:vec.rs-0305 */     }
/* FP:vec.rs-0306 */ }
/* FP:vec.rs-0307 */ 
/* FP:vec.rs-0308 */ impl<'a, I: Idx, T> IntoIterator for &'a mut IndexVec<I, T> {
/* FP:vec.rs-0309 */     type Item = &'a mut T;
/* FP:vec.rs-0310 */     type IntoIter = slice::IterMut<'a, T>;
/* FP:vec.rs-0311 */ 
/* FP:vec.rs-0312 */     #[inline]
/* FP:vec.rs-0313 */     fn into_iter(self) -> slice::IterMut<'a, T> {
/* FP:vec.rs-0314 */         self.iter_mut()
/* FP:vec.rs-0315 */     }
/* FP:vec.rs-0316 */ }
/* FP:vec.rs-0317 */ 
/* FP:vec.rs-0318 */ impl<I: Idx, T> Default for IndexVec<I, T> {
/* FP:vec.rs-0319 */     #[inline]
/* FP:vec.rs-0320 */     fn default() -> Self {
/* FP:vec.rs-0321 */         IndexVec::new()
/* FP:vec.rs-0322 */     }
/* FP:vec.rs-0323 */ }
/* FP:vec.rs-0324 */ 
/* FP:vec.rs-0325 */ impl<I: Idx, T, const N: usize> From<[T; N]> for IndexVec<I, T> {
/* FP:vec.rs-0326 */     #[inline]
/* FP:vec.rs-0327 */     fn from(array: [T; N]) -> Self {
/* FP:vec.rs-0328 */         IndexVec::from_raw(array.into())
/* FP:vec.rs-0329 */     }
/* FP:vec.rs-0330 */ }
/* FP:vec.rs-0331 */ 
/* FP:vec.rs-0332 */ #[cfg(feature = "nightly")]
/* FP:vec.rs-0333 */ impl<S: Encoder, I: Idx, T: Encodable<S>> Encodable<S> for IndexVec<I, T> {
/* FP:vec.rs-0334 */     fn encode(&self, s: &mut S) {
/* FP:vec.rs-0335 */         Encodable::encode(&self.raw, s);
/* FP:vec.rs-0336 */     }
/* FP:vec.rs-0337 */ }
/* FP:vec.rs-0338 */ 
/* FP:vec.rs-0339 */ #[cfg(feature = "nightly")]
/* FP:vec.rs-0340 */ impl<D: Decoder, I: Idx, T: Decodable<D>> Decodable<D> for IndexVec<I, T> {
/* FP:vec.rs-0341 */     fn decode(d: &mut D) -> Self {
/* FP:vec.rs-0342 */         IndexVec::from_raw(Vec::<T>::decode(d))
/* FP:vec.rs-0343 */     }
/* FP:vec.rs-0344 */ }
/* FP:vec.rs-0345 */ 
/* FP:vec.rs-0346 */ // Whether `IndexVec` is `Send` depends only on the data,
/* FP:vec.rs-0347 */ // not the phantom data.
/* FP:vec.rs-0348 */ unsafe impl<I: Idx, T> Send for IndexVec<I, T> where T: Send {}
/* FP:vec.rs-0349 */ 
/* FP:vec.rs-0350 */ #[cfg(test)]