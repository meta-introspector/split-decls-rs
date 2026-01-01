/* FP:bit_set.rs-0001 */ use std::marker::PhantomData;
/* FP:bit_set.rs-0002 */ #[cfg(not(feature = "nightly"))]
/* FP:bit_set.rs-0003 */ use std::mem;
/* FP:bit_set.rs-0004 */ use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Bound, Not, Range, RangeBounds, Shl};
/* FP:bit_set.rs-0005 */ use std::rc::Rc;
/* FP:bit_set.rs-0006 */ use std::{fmt, iter, slice};
/* FP:bit_set.rs-0007 */ 
/* FP:bit_set.rs-0008 */ use Chunk::*;
/* FP:bit_set.rs-0009 */ #[cfg(feature = "nightly")]
/* FP:bit_set.rs-0010 */ use rustc_macros::{Decodable_NoContext, Encodable_NoContext};
/* FP:bit_set.rs-0011 */ use smallvec::{SmallVec, smallvec};
/* FP:bit_set.rs-0012 */ 
/* FP:bit_set.rs-0013 */ use crate::{Idx, IndexVec};
/* FP:bit_set.rs-0014 */ 
/* FP:bit_set.rs-0015 */ #[cfg(test)]
/* FP:bit_set.rs-0017 */ 
/* FP:bit_set.rs-0018 */ type Word = u64;
/* FP:bit_set.rs-0019 */ const WORD_BYTES: usize = size_of::<Word>();
/* FP:bit_set.rs-0020 */ const WORD_BITS: usize = WORD_BYTES * 8;
/* FP:bit_set.rs-0021 */ 
/* FP:bit_set.rs-0022 */ // The choice of chunk size has some trade-offs.
/* FP:bit_set.rs-0023 */ //
/* FP:bit_set.rs-0024 */ // A big chunk size tends to favour cases where many large `ChunkedBitSet`s are
/* FP:bit_set.rs-0025 */ // present, because they require fewer `Chunk`s, reducing the number of
/* FP:bit_set.rs-0026 */ // allocations and reducing peak memory usage. Also, fewer chunk operations are
/* FP:bit_set.rs-0027 */ // required, though more of them might be `Mixed`.
/* FP:bit_set.rs-0028 */ //
/* FP:bit_set.rs-0029 */ // A small chunk size tends to favour cases where many small `ChunkedBitSet`s
/* FP:bit_set.rs-0030 */ // are present, because less space is wasted at the end of the final chunk (if
/* FP:bit_set.rs-0031 */ // it's not full).
/* FP:bit_set.rs-0032 */ const CHUNK_WORDS: usize = 32;
/* FP:bit_set.rs-0033 */ const CHUNK_BITS: usize = CHUNK_WORDS * WORD_BITS; // 2048 bits
/* FP:bit_set.rs-0034 */ 
/* FP:bit_set.rs-0035 */ /// ChunkSize is small to keep `Chunk` small. The static assertion ensures it's
/* FP:bit_set.rs-0036 */ /// not too small.
/* FP:bit_set.rs-0037 */ type ChunkSize = u16;
/* FP:bit_set.rs-0038 */ const _: () = assert!(CHUNK_BITS <= ChunkSize::MAX as usize);
/* FP:bit_set.rs-0039 */ 
/* FP:bit_set.rs-0040 */ pub trait BitRelations<Rhs> {
/* FP:bit_set.rs-0041 */     fn union(&mut self, other: &Rhs) -> bool;
/* FP:bit_set.rs-0042 */     fn subtract(&mut self, other: &Rhs) -> bool;
/* FP:bit_set.rs-0043 */     fn intersect(&mut self, other: &Rhs) -> bool;
/* FP:bit_set.rs-0044 */ }
/* FP:bit_set.rs-0045 */ 
/* FP:bit_set.rs-0046 */ #[inline]
/* FP:bit_set.rs-0047 */ fn inclusive_start_end<T: Idx>(
/* FP:bit_set.rs-0048 */     range: impl RangeBounds<T>,
/* FP:bit_set.rs-0049 */     domain: usize,
/* FP:bit_set.rs-0050 */ ) -> Option<(usize, usize)> {
/* FP:bit_set.rs-0051 */     // Both start and end are inclusive.
/* FP:bit_set.rs-0052 */     let start = match range.start_bound().cloned() {
/* FP:bit_set.rs-0053 */         Bound::Included(start) => start.index(),
/* FP:bit_set.rs-0054 */         Bound::Excluded(start) => start.index() + 1,
/* FP:bit_set.rs-0055 */         Bound::Unbounded => 0,
/* FP:bit_set.rs-0056 */     };
/* FP:bit_set.rs-0057 */     let end = match range.end_bound().cloned() {
/* FP:bit_set.rs-0058 */         Bound::Included(end) => end.index(),
/* FP:bit_set.rs-0059 */         Bound::Excluded(end) => end.index().checked_sub(1)?,
/* FP:bit_set.rs-0060 */         Bound::Unbounded => domain - 1,
/* FP:bit_set.rs-0061 */     };
/* FP:bit_set.rs-0062 */     assert!(end < domain);
/* FP:bit_set.rs-0063 */     if start > end {
/* FP:bit_set.rs-0064 */         return None;
/* FP:bit_set.rs-0065 */     }
/* FP:bit_set.rs-0066 */     Some((start, end))
/* FP:bit_set.rs-0067 */ }
/* FP:bit_set.rs-0068 */ 
/* FP:bit_set.rs-0069 */ macro_rules! bit_relations_inherent_impls {
/* FP:bit_set.rs-0070 */     () => {
/* FP:bit_set.rs-0071 */         /// Sets `self = self | other` and returns `true` if `self` changed
/* FP:bit_set.rs-0072 */         /// (i.e., if new bits were added).
/* FP:bit_set.rs-0073 */         pub fn union<Rhs>(&mut self, other: &Rhs) -> bool
/* FP:bit_set.rs-0074 */         where
/* FP:bit_set.rs-0075 */             Self: BitRelations<Rhs>,
/* FP:bit_set.rs-0076 */         {
/* FP:bit_set.rs-0077 */             <Self as BitRelations<Rhs>>::union(self, other)
/* FP:bit_set.rs-0078 */         }
/* FP:bit_set.rs-0079 */ 
/* FP:bit_set.rs-0080 */         /// Sets `self = self - other` and returns `true` if `self` changed.
/* FP:bit_set.rs-0081 */         /// (i.e., if any bits were removed).
/* FP:bit_set.rs-0082 */         pub fn subtract<Rhs>(&mut self, other: &Rhs) -> bool
/* FP:bit_set.rs-0083 */         where
/* FP:bit_set.rs-0084 */             Self: BitRelations<Rhs>,
/* FP:bit_set.rs-0085 */         {
/* FP:bit_set.rs-0086 */             <Self as BitRelations<Rhs>>::subtract(self, other)
/* FP:bit_set.rs-0087 */         }
/* FP:bit_set.rs-0088 */ 
/* FP:bit_set.rs-0089 */         /// Sets `self = self & other` and return `true` if `self` changed.
/* FP:bit_set.rs-0090 */         /// (i.e., if any bits were removed).
/* FP:bit_set.rs-0091 */         pub fn intersect<Rhs>(&mut self, other: &Rhs) -> bool
/* FP:bit_set.rs-0092 */         where
/* FP:bit_set.rs-0093 */             Self: BitRelations<Rhs>,
/* FP:bit_set.rs-0094 */         {
/* FP:bit_set.rs-0095 */             <Self as BitRelations<Rhs>>::intersect(self, other)
/* FP:bit_set.rs-0096 */         }
/* FP:bit_set.rs-0097 */     };
/* FP:bit_set.rs-0098 */ }
/* FP:bit_set.rs-0099 */ 
/* FP:bit_set.rs-0100 */ /// A fixed-size bitset type with a dense representation.
/* FP:bit_set.rs-0101 */ ///
/* FP:bit_set.rs-0102 */ /// Note 1: Since this bitset is dense, if your domain is big, and/or relatively
/* FP:bit_set.rs-0103 */ /// homogeneous (for example, with long runs of bits set or unset), then it may
/* FP:bit_set.rs-0104 */ /// be preferable to instead use a [MixedBitSet], or an
/* FP:bit_set.rs-0105 */ /// [IntervalSet](crate::interval::IntervalSet). They should be more suited to
/* FP:bit_set.rs-0106 */ /// sparse, or highly-compressible, domains.
/* FP:bit_set.rs-0107 */ ///
/* FP:bit_set.rs-0108 */ /// Note 2: Use [`GrowableBitSet`] if you need support for resizing after creation.
/* FP:bit_set.rs-0109 */ ///
/* FP:bit_set.rs-0110 */ /// `T` is an index type, typically a newtyped `usize` wrapper, but it can also
/* FP:bit_set.rs-0111 */ /// just be `usize`.
/* FP:bit_set.rs-0112 */ ///
/* FP:bit_set.rs-0113 */ /// All operations that involve an element will panic if the element is equal
/* FP:bit_set.rs-0114 */ /// to or greater than the domain size. All operations that involve two bitsets
/* FP:bit_set.rs-0115 */ /// will panic if the bitsets have differing domain sizes.
/* FP:bit_set.rs-0116 */ ///
/* FP:bit_set.rs-0117 */ #[cfg_attr(feature = "nightly", derive(Decodable_NoContext, Encodable_NoContext))]
/* FP:bit_set.rs-0118 */ #[derive(Eq, PartialEq, Hash)]
/* FP:bit_set.rs-0119 */ pub struct DenseBitSet<T> {
/* FP:bit_set.rs-0120 */     domain_size: usize,
/* FP:bit_set.rs-0121 */     words: SmallVec<[Word; 2]>,
/* FP:bit_set.rs-0122 */     marker: PhantomData<T>,
/* FP:bit_set.rs-0123 */ }
/* FP:bit_set.rs-0124 */ 
/* FP:bit_set.rs-0125 */ impl<T> DenseBitSet<T> {
/* FP:bit_set.rs-0126 */     /// Gets the domain size.
/* FP:bit_set.rs-0127 */     pub fn domain_size(&self) -> usize {
/* FP:bit_set.rs-0128 */         self.domain_size
/* FP:bit_set.rs-0129 */     }
/* FP:bit_set.rs-0130 */ }
/* FP:bit_set.rs-0131 */ 
/* FP:bit_set.rs-0132 */ impl<T: Idx> DenseBitSet<T> {
/* FP:bit_set.rs-0133 */     /// Creates a new, empty bitset with a given `domain_size`.
/* FP:bit_set.rs-0134 */     #[inline]
/* FP:bit_set.rs-0135 */     pub fn new_empty(domain_size: usize) -> DenseBitSet<T> {
/* FP:bit_set.rs-0136 */         let num_words = num_words(domain_size);
/* FP:bit_set.rs-0137 */         DenseBitSet { domain_size, words: smallvec![0; num_words], marker: PhantomData }
/* FP:bit_set.rs-0138 */     }
/* FP:bit_set.rs-0139 */ 
/* FP:bit_set.rs-0140 */     /// Creates a new, filled bitset with a given `domain_size`.
/* FP:bit_set.rs-0141 */     #[inline]
/* FP:bit_set.rs-0142 */     pub fn new_filled(domain_size: usize) -> DenseBitSet<T> {
/* FP:bit_set.rs-0143 */         let num_words = num_words(domain_size);
/* FP:bit_set.rs-0144 */         let mut result =
/* FP:bit_set.rs-0145 */             DenseBitSet { domain_size, words: smallvec![!0; num_words], marker: PhantomData };
/* FP:bit_set.rs-0146 */         result.clear_excess_bits();
/* FP:bit_set.rs-0147 */         result
/* FP:bit_set.rs-0148 */     }
/* FP:bit_set.rs-0149 */ 
/* FP:bit_set.rs-0150 */     /// Clear all elements.
/* FP:bit_set.rs-0151 */     #[inline]
/* FP:bit_set.rs-0152 */     pub fn clear(&mut self) {
/* FP:bit_set.rs-0153 */         self.words.fill(0);
/* FP:bit_set.rs-0154 */     }
/* FP:bit_set.rs-0155 */ 
/* FP:bit_set.rs-0156 */     /// Clear excess bits in the final word.
/* FP:bit_set.rs-0157 */     fn clear_excess_bits(&mut self) {
/* FP:bit_set.rs-0158 */         clear_excess_bits_in_final_word(self.domain_size, &mut self.words);
/* FP:bit_set.rs-0159 */     }
/* FP:bit_set.rs-0160 */ 
/* FP:bit_set.rs-0161 */     /// Count the number of set bits in the set.
/* FP:bit_set.rs-0162 */     pub fn count(&self) -> usize {
/* FP:bit_set.rs-0163 */         self.words.iter().map(|e| e.count_ones() as usize).sum()
/* FP:bit_set.rs-0164 */     }
/* FP:bit_set.rs-0165 */ 
/* FP:bit_set.rs-0166 */     /// Returns `true` if `self` contains `elem`.
/* FP:bit_set.rs-0167 */     #[inline]
/* FP:bit_set.rs-0168 */     pub fn contains(&self, elem: T) -> bool {
/* FP:bit_set.rs-0169 */         assert!(elem.index() < self.domain_size);
/* FP:bit_set.rs-0170 */         let (word_index, mask) = word_index_and_mask(elem);
/* FP:bit_set.rs-0171 */         (self.words[word_index] & mask) != 0
/* FP:bit_set.rs-0172 */     }
/* FP:bit_set.rs-0173 */ 
/* FP:bit_set.rs-0174 */     /// Is `self` is a (non-strict) superset of `other`?
/* FP:bit_set.rs-0175 */     #[inline]
/* FP:bit_set.rs-0176 */     pub fn superset(&self, other: &DenseBitSet<T>) -> bool {
/* FP:bit_set.rs-0177 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0178 */         self.words.iter().zip(&other.words).all(|(a, b)| (a & b) == *b)
/* FP:bit_set.rs-0179 */     }
/* FP:bit_set.rs-0180 */ 
/* FP:bit_set.rs-0181 */     /// Is the set empty?
/* FP:bit_set.rs-0182 */     #[inline]
/* FP:bit_set.rs-0183 */     pub fn is_empty(&self) -> bool {
/* FP:bit_set.rs-0184 */         self.words.iter().all(|a| *a == 0)
/* FP:bit_set.rs-0185 */     }
/* FP:bit_set.rs-0186 */ 
/* FP:bit_set.rs-0187 */     /// Insert `elem`. Returns whether the set has changed.
/* FP:bit_set.rs-0188 */     #[inline]
/* FP:bit_set.rs-0189 */     pub fn insert(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-0190 */         assert!(
/* FP:bit_set.rs-0191 */             elem.index() < self.domain_size,
/* FP:bit_set.rs-0192 */             "inserting element at index {} but domain size is {}",
/* FP:bit_set.rs-0193 */             elem.index(),
/* FP:bit_set.rs-0194 */             self.domain_size,
/* FP:bit_set.rs-0195 */         );
/* FP:bit_set.rs-0196 */         let (word_index, mask) = word_index_and_mask(elem);
/* FP:bit_set.rs-0197 */         let word_ref = &mut self.words[word_index];
/* FP:bit_set.rs-0198 */         let word = *word_ref;
/* FP:bit_set.rs-0199 */         let new_word = word | mask;
/* FP:bit_set.rs-0200 */         *word_ref = new_word;
/* FP:bit_set.rs-0201 */         new_word != word
/* FP:bit_set.rs-0202 */     }
/* FP:bit_set.rs-0203 */ 
/* FP:bit_set.rs-0204 */     #[inline]
/* FP:bit_set.rs-0205 */     pub fn insert_range(&mut self, elems: impl RangeBounds<T>) {
/* FP:bit_set.rs-0206 */         let Some((start, end)) = inclusive_start_end(elems, self.domain_size) else {
/* FP:bit_set.rs-0207 */             return;
/* FP:bit_set.rs-0208 */         };
/* FP:bit_set.rs-0209 */ 
/* FP:bit_set.rs-0210 */         let (start_word_index, start_mask) = word_index_and_mask(start);
/* FP:bit_set.rs-0211 */         let (end_word_index, end_mask) = word_index_and_mask(end);
/* FP:bit_set.rs-0212 */ 
/* FP:bit_set.rs-0213 */         // Set all words in between start and end (exclusively of both).
/* FP:bit_set.rs-0214 */         for word_index in (start_word_index + 1)..end_word_index {
/* FP:bit_set.rs-0215 */             self.words[word_index] = !0;
/* FP:bit_set.rs-0216 */         }
/* FP:bit_set.rs-0217 */ 
/* FP:bit_set.rs-0218 */         if start_word_index != end_word_index {
/* FP:bit_set.rs-0219 */             // Start and end are in different words, so we handle each in turn.
/* FP:bit_set.rs-0220 */             //
/* FP:bit_set.rs-0221 */             // We set all leading bits. This includes the start_mask bit.
/* FP:bit_set.rs-0222 */             self.words[start_word_index] |= !(start_mask - 1);
/* FP:bit_set.rs-0223 */             // And all trailing bits (i.e. from 0..=end) in the end word,
/* FP:bit_set.rs-0224 */             // including the end.
/* FP:bit_set.rs-0225 */             self.words[end_word_index] |= end_mask | (end_mask - 1);
/* FP:bit_set.rs-0226 */         } else {
/* FP:bit_set.rs-0227 */             self.words[start_word_index] |= end_mask | (end_mask - start_mask);
/* FP:bit_set.rs-0228 */         }
/* FP:bit_set.rs-0229 */     }
/* FP:bit_set.rs-0230 */ 
/* FP:bit_set.rs-0231 */     /// Sets all bits to true.
/* FP:bit_set.rs-0232 */     pub fn insert_all(&mut self) {
/* FP:bit_set.rs-0233 */         self.words.fill(!0);
/* FP:bit_set.rs-0234 */         self.clear_excess_bits();
/* FP:bit_set.rs-0235 */     }
/* FP:bit_set.rs-0236 */ 
/* FP:bit_set.rs-0237 */     /// Checks whether any bit in the given range is a 1.
/* FP:bit_set.rs-0238 */     #[inline]
/* FP:bit_set.rs-0239 */     pub fn contains_any(&self, elems: impl RangeBounds<T>) -> bool {
/* FP:bit_set.rs-0240 */         let Some((start, end)) = inclusive_start_end(elems, self.domain_size) else {
/* FP:bit_set.rs-0241 */             return false;
/* FP:bit_set.rs-0242 */         };
/* FP:bit_set.rs-0243 */         let (start_word_index, start_mask) = word_index_and_mask(start);
/* FP:bit_set.rs-0244 */         let (end_word_index, end_mask) = word_index_and_mask(end);
/* FP:bit_set.rs-0245 */ 
/* FP:bit_set.rs-0246 */         if start_word_index == end_word_index {
/* FP:bit_set.rs-0247 */             self.words[start_word_index] & (end_mask | (end_mask - start_mask)) != 0
/* FP:bit_set.rs-0248 */         } else {
/* FP:bit_set.rs-0249 */             if self.words[start_word_index] & !(start_mask - 1) != 0 {
/* FP:bit_set.rs-0250 */                 return true;
/* FP:bit_set.rs-0251 */             }
/* FP:bit_set.rs-0252 */ 
/* FP:bit_set.rs-0253 */             let remaining = start_word_index + 1..end_word_index;
/* FP:bit_set.rs-0254 */             if remaining.start <= remaining.end {
/* FP:bit_set.rs-0255 */                 self.words[remaining].iter().any(|&w| w != 0)
/* FP:bit_set.rs-0256 */                     || self.words[end_word_index] & (end_mask | (end_mask - 1)) != 0
/* FP:bit_set.rs-0257 */             } else {
/* FP:bit_set.rs-0258 */                 false
/* FP:bit_set.rs-0259 */             }
/* FP:bit_set.rs-0260 */         }
/* FP:bit_set.rs-0261 */     }
/* FP:bit_set.rs-0262 */ 
/* FP:bit_set.rs-0263 */     /// Returns `true` if the set has changed.
/* FP:bit_set.rs-0264 */     #[inline]
/* FP:bit_set.rs-0265 */     pub fn remove(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-0266 */         assert!(elem.index() < self.domain_size);
/* FP:bit_set.rs-0267 */         let (word_index, mask) = word_index_and_mask(elem);
/* FP:bit_set.rs-0268 */         let word_ref = &mut self.words[word_index];
/* FP:bit_set.rs-0269 */         let word = *word_ref;
/* FP:bit_set.rs-0270 */         let new_word = word & !mask;
/* FP:bit_set.rs-0271 */         *word_ref = new_word;
/* FP:bit_set.rs-0272 */         new_word != word
/* FP:bit_set.rs-0273 */     }
/* FP:bit_set.rs-0274 */ 
/* FP:bit_set.rs-0275 */     /// Iterates over the indices of set bits in a sorted order.
/* FP:bit_set.rs-0276 */     #[inline]
/* FP:bit_set.rs-0277 */     pub fn iter(&self) -> BitIter<'_, T> {
/* FP:bit_set.rs-0278 */         BitIter::new(&self.words)
/* FP:bit_set.rs-0279 */     }
/* FP:bit_set.rs-0280 */ 
/* FP:bit_set.rs-0281 */     pub fn last_set_in(&self, range: impl RangeBounds<T>) -> Option<T> {
/* FP:bit_set.rs-0282 */         let (start, end) = inclusive_start_end(range, self.domain_size)?;
/* FP:bit_set.rs-0283 */         let (start_word_index, _) = word_index_and_mask(start);
/* FP:bit_set.rs-0284 */         let (end_word_index, end_mask) = word_index_and_mask(end);
/* FP:bit_set.rs-0285 */ 
/* FP:bit_set.rs-0286 */         let end_word = self.words[end_word_index] & (end_mask | (end_mask - 1));
/* FP:bit_set.rs-0287 */         if end_word != 0 {
/* FP:bit_set.rs-0288 */             let pos = max_bit(end_word) + WORD_BITS * end_word_index;
/* FP:bit_set.rs-0289 */             if start <= pos {
/* FP:bit_set.rs-0290 */                 return Some(T::new(pos));
/* FP:bit_set.rs-0291 */             }
/* FP:bit_set.rs-0292 */         }
/* FP:bit_set.rs-0293 */ 
/* FP:bit_set.rs-0294 */         // We exclude end_word_index from the range here, because we don't want
/* FP:bit_set.rs-0295 */         // to limit ourselves to *just* the last word: the bits set it in may be
/* FP:bit_set.rs-0296 */         // after `end`, so it may not work out.
/* FP:bit_set.rs-0297 */         if let Some(offset) =
/* FP:bit_set.rs-0298 */             self.words[start_word_index..end_word_index].iter().rposition(|&w| w != 0)
/* FP:bit_set.rs-0299 */         {
/* FP:bit_set.rs-0300 */             let word_idx = start_word_index + offset;
/* FP:bit_set.rs-0301 */             let start_word = self.words[word_idx];
/* FP:bit_set.rs-0302 */             let pos = max_bit(start_word) + WORD_BITS * word_idx;
/* FP:bit_set.rs-0303 */             if start <= pos {
/* FP:bit_set.rs-0304 */                 return Some(T::new(pos));
/* FP:bit_set.rs-0305 */             }
/* FP:bit_set.rs-0306 */         }
/* FP:bit_set.rs-0307 */ 
/* FP:bit_set.rs-0308 */         None
/* FP:bit_set.rs-0309 */     }
/* FP:bit_set.rs-0310 */ 
/* FP:bit_set.rs-0311 */     bit_relations_inherent_impls! {}
/* FP:bit_set.rs-0312 */ 
/* FP:bit_set.rs-0313 */     /// Sets `self = self | !other`.
/* FP:bit_set.rs-0314 */     ///
/* FP:bit_set.rs-0315 */     /// FIXME: Incorporate this into [`BitRelations`] and fill out
/* FP:bit_set.rs-0316 */     /// implementations for other bitset types, if needed.
/* FP:bit_set.rs-0317 */     pub fn union_not(&mut self, other: &DenseBitSet<T>) {
/* FP:bit_set.rs-0318 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0319 */ 
/* FP:bit_set.rs-0320 */         // FIXME(Zalathar): If we were to forcibly _set_ all excess bits before
/* FP:bit_set.rs-0321 */         // the bitwise update, and then clear them again afterwards, we could
/* FP:bit_set.rs-0322 */         // quickly and accurately detect whether the update changed anything.
/* FP:bit_set.rs-0323 */         // But that's only worth doing if there's an actual use-case.
/* FP:bit_set.rs-0324 */ 
/* FP:bit_set.rs-0325 */         bitwise(&mut self.words, &other.words, |a, b| a | !b);
/* FP:bit_set.rs-0326 */         // The bitwise update `a | !b` can result in the last word containing
/* FP:bit_set.rs-0327 */         // out-of-domain bits, so we need to clear them.
/* FP:bit_set.rs-0328 */         self.clear_excess_bits();
/* FP:bit_set.rs-0329 */     }
/* FP:bit_set.rs-0330 */ }
/* FP:bit_set.rs-0331 */ 
/* FP:bit_set.rs-0332 */ // dense REL dense
/* FP:bit_set.rs-0333 */ impl<T: Idx> BitRelations<DenseBitSet<T>> for DenseBitSet<T> {
/* FP:bit_set.rs-0334 */     fn union(&mut self, other: &DenseBitSet<T>) -> bool {
/* FP:bit_set.rs-0335 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0336 */         bitwise(&mut self.words, &other.words, |a, b| a | b)
/* FP:bit_set.rs-0337 */     }
/* FP:bit_set.rs-0338 */ 
/* FP:bit_set.rs-0339 */     fn subtract(&mut self, other: &DenseBitSet<T>) -> bool {
/* FP:bit_set.rs-0340 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0341 */         bitwise(&mut self.words, &other.words, |a, b| a & !b)
/* FP:bit_set.rs-0342 */     }
/* FP:bit_set.rs-0343 */ 
/* FP:bit_set.rs-0344 */     fn intersect(&mut self, other: &DenseBitSet<T>) -> bool {
/* FP:bit_set.rs-0345 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0346 */         bitwise(&mut self.words, &other.words, |a, b| a & b)
/* FP:bit_set.rs-0347 */     }
/* FP:bit_set.rs-0348 */ }
/* FP:bit_set.rs-0349 */ 
/* FP:bit_set.rs-0350 */ impl<T: Idx> From<GrowableBitSet<T>> for DenseBitSet<T> {
/* FP:bit_set.rs-0351 */     fn from(bit_set: GrowableBitSet<T>) -> Self {
/* FP:bit_set.rs-0352 */         bit_set.bit_set
/* FP:bit_set.rs-0353 */     }
/* FP:bit_set.rs-0354 */ }
/* FP:bit_set.rs-0355 */ 
/* FP:bit_set.rs-0356 */ impl<T> Clone for DenseBitSet<T> {
/* FP:bit_set.rs-0357 */     fn clone(&self) -> Self {
/* FP:bit_set.rs-0358 */         DenseBitSet {
/* FP:bit_set.rs-0359 */             domain_size: self.domain_size,
/* FP:bit_set.rs-0360 */             words: self.words.clone(),
/* FP:bit_set.rs-0361 */             marker: PhantomData,
/* FP:bit_set.rs-0362 */         }
/* FP:bit_set.rs-0363 */     }
/* FP:bit_set.rs-0364 */ 
/* FP:bit_set.rs-0365 */     fn clone_from(&mut self, from: &Self) {
/* FP:bit_set.rs-0366 */         self.domain_size = from.domain_size;
/* FP:bit_set.rs-0367 */         self.words.clone_from(&from.words);
/* FP:bit_set.rs-0368 */     }
/* FP:bit_set.rs-0369 */ }
/* FP:bit_set.rs-0370 */ 
/* FP:bit_set.rs-0371 */ impl<T: Idx> fmt::Debug for DenseBitSet<T> {
/* FP:bit_set.rs-0372 */     fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-0373 */         w.debug_list().entries(self.iter()).finish()
/* FP:bit_set.rs-0374 */     }
/* FP:bit_set.rs-0375 */ }
/* FP:bit_set.rs-0376 */ 
/* FP:bit_set.rs-0377 */ impl<T: Idx> ToString for DenseBitSet<T> {
/* FP:bit_set.rs-0378 */     fn to_string(&self) -> String {
/* FP:bit_set.rs-0379 */         let mut result = String::new();
/* FP:bit_set.rs-0380 */         let mut sep = '[';
/* FP:bit_set.rs-0381 */ 
/* FP:bit_set.rs-0382 */         // Note: this is a little endian printout of bytes.
/* FP:bit_set.rs-0383 */ 
/* FP:bit_set.rs-0384 */         // i tracks how many bits we have printed so far.
/* FP:bit_set.rs-0385 */         let mut i = 0;
/* FP:bit_set.rs-0386 */         for word in &self.words {
/* FP:bit_set.rs-0387 */             let mut word = *word;
/* FP:bit_set.rs-0388 */             for _ in 0..WORD_BYTES {
/* FP:bit_set.rs-0389 */                 // for each byte in `word`:
/* FP:bit_set.rs-0390 */                 let remain = self.domain_size - i;
/* FP:bit_set.rs-0391 */                 // If less than a byte remains, then mask just that many bits.
/* FP:bit_set.rs-0392 */                 let mask = if remain <= 8 { (1 << remain) - 1 } else { 0xFF };
/* FP:bit_set.rs-0393 */                 assert!(mask <= 0xFF);
/* FP:bit_set.rs-0394 */                 let byte = word & mask;
/* FP:bit_set.rs-0395 */ 
/* FP:bit_set.rs-0396 */                 result.push_str(&format!("{sep}{byte:02x}"));
/* FP:bit_set.rs-0397 */ 
/* FP:bit_set.rs-0398 */                 if remain <= 8 {
/* FP:bit_set.rs-0399 */                     break;
/* FP:bit_set.rs-0400 */                 }
/* FP:bit_set.rs-0401 */                 word >>= 8;
/* FP:bit_set.rs-0402 */                 i += 8;
/* FP:bit_set.rs-0403 */                 sep = '-';
/* FP:bit_set.rs-0404 */             }
/* FP:bit_set.rs-0405 */             sep = '|';
/* FP:bit_set.rs-0406 */         }
/* FP:bit_set.rs-0407 */         result.push(']');
/* FP:bit_set.rs-0408 */ 
/* FP:bit_set.rs-0409 */         result
/* FP:bit_set.rs-0410 */     }
/* FP:bit_set.rs-0411 */ }
/* FP:bit_set.rs-0412 */ 
/* FP:bit_set.rs-0413 */ pub struct BitIter<'a, T: Idx> {
/* FP:bit_set.rs-0414 */     /// A copy of the current word, but with any already-visited bits cleared.
/* FP:bit_set.rs-0415 */     /// (This lets us use `trailing_zeros()` to find the next set bit.) When it
/* FP:bit_set.rs-0416 */     /// is reduced to 0, we move onto the next word.
/* FP:bit_set.rs-0417 */     word: Word,
/* FP:bit_set.rs-0418 */ 
/* FP:bit_set.rs-0419 */     /// The offset (measured in bits) of the current word.
/* FP:bit_set.rs-0420 */     offset: usize,
/* FP:bit_set.rs-0421 */ 
/* FP:bit_set.rs-0422 */     /// Underlying iterator over the words.
/* FP:bit_set.rs-0423 */     iter: slice::Iter<'a, Word>,
/* FP:bit_set.rs-0424 */ 
/* FP:bit_set.rs-0425 */     marker: PhantomData<T>,
/* FP:bit_set.rs-0426 */ }
/* FP:bit_set.rs-0427 */ 
/* FP:bit_set.rs-0428 */ impl<'a, T: Idx> BitIter<'a, T> {
/* FP:bit_set.rs-0429 */     #[inline]
/* FP:bit_set.rs-0430 */     fn new(words: &'a [Word]) -> BitIter<'a, T> {
/* FP:bit_set.rs-0431 */         // We initialize `word` and `offset` to degenerate values. On the first
/* FP:bit_set.rs-0432 */         // call to `next()` we will fall through to getting the first word from
/* FP:bit_set.rs-0433 */         // `iter`, which sets `word` to the first word (if there is one) and
/* FP:bit_set.rs-0434 */         // `offset` to 0. Doing it this way saves us from having to maintain
/* FP:bit_set.rs-0435 */         // additional state about whether we have started.
/* FP:bit_set.rs-0436 */         BitIter {
/* FP:bit_set.rs-0437 */             word: 0,
/* FP:bit_set.rs-0438 */             offset: usize::MAX - (WORD_BITS - 1),
/* FP:bit_set.rs-0439 */             iter: words.iter(),
/* FP:bit_set.rs-0440 */             marker: PhantomData,
/* FP:bit_set.rs-0441 */         }
/* FP:bit_set.rs-0442 */     }
/* FP:bit_set.rs-0443 */ }
/* FP:bit_set.rs-0444 */ 
/* FP:bit_set.rs-0445 */ impl<'a, T: Idx> Iterator for BitIter<'a, T> {
/* FP:bit_set.rs-0446 */     type Item = T;
/* FP:bit_set.rs-0447 */     fn next(&mut self) -> Option<T> {
/* FP:bit_set.rs-0448 */         loop {
/* FP:bit_set.rs-0449 */             if self.word != 0 {
/* FP:bit_set.rs-0450 */                 // Get the position of the next set bit in the current word,
/* FP:bit_set.rs-0451 */                 // then clear the bit.
/* FP:bit_set.rs-0452 */                 let bit_pos = self.word.trailing_zeros() as usize;
/* FP:bit_set.rs-0453 */                 self.word ^= 1 << bit_pos;
/* FP:bit_set.rs-0454 */                 return Some(T::new(bit_pos + self.offset));
/* FP:bit_set.rs-0455 */             }
/* FP:bit_set.rs-0456 */ 
/* FP:bit_set.rs-0457 */             // Move onto the next word. `wrapping_add()` is needed to handle
/* FP:bit_set.rs-0458 */             // the degenerate initial value given to `offset` in `new()`.
/* FP:bit_set.rs-0459 */             self.word = *self.iter.next()?;
/* FP:bit_set.rs-0460 */             self.offset = self.offset.wrapping_add(WORD_BITS);
/* FP:bit_set.rs-0461 */         }
/* FP:bit_set.rs-0462 */     }
/* FP:bit_set.rs-0463 */ }
/* FP:bit_set.rs-0464 */ 
/* FP:bit_set.rs-0465 */ /// A fixed-size bitset type with a partially dense, partially sparse
/* FP:bit_set.rs-0466 */ /// representation. The bitset is broken into chunks, and chunks that are all
/* FP:bit_set.rs-0467 */ /// zeros or all ones are represented and handled very efficiently.
/* FP:bit_set.rs-0468 */ ///
/* FP:bit_set.rs-0469 */ /// This type is especially efficient for sets that typically have a large
/* FP:bit_set.rs-0470 */ /// `domain_size` with significant stretches of all zeros or all ones, and also
/* FP:bit_set.rs-0471 */ /// some stretches with lots of 0s and 1s mixed in a way that causes trouble
/* FP:bit_set.rs-0472 */ /// for `IntervalSet`.
/* FP:bit_set.rs-0473 */ ///
/* FP:bit_set.rs-0474 */ /// Best used via `MixedBitSet`, rather than directly, because `MixedBitSet`
/* FP:bit_set.rs-0475 */ /// has better performance for small bitsets.
/* FP:bit_set.rs-0476 */ ///
/* FP:bit_set.rs-0477 */ /// `T` is an index type, typically a newtyped `usize` wrapper, but it can also
/* FP:bit_set.rs-0478 */ /// just be `usize`.
/* FP:bit_set.rs-0479 */ ///
/* FP:bit_set.rs-0480 */ /// All operations that involve an element will panic if the element is equal
/* FP:bit_set.rs-0481 */ /// to or greater than the domain size. All operations that involve two bitsets
/* FP:bit_set.rs-0482 */ /// will panic if the bitsets have differing domain sizes.
/* FP:bit_set.rs-0483 */ #[derive(PartialEq, Eq)]
/* FP:bit_set.rs-0484 */ pub struct ChunkedBitSet<T> {
/* FP:bit_set.rs-0485 */     domain_size: usize,
/* FP:bit_set.rs-0486 */ 
/* FP:bit_set.rs-0487 */     /// The chunks. Each one contains exactly CHUNK_BITS values, except the
/* FP:bit_set.rs-0488 */     /// last one which contains 1..=CHUNK_BITS values.
/* FP:bit_set.rs-0489 */     chunks: Box<[Chunk]>,
/* FP:bit_set.rs-0490 */ 
/* FP:bit_set.rs-0491 */     marker: PhantomData<T>,
/* FP:bit_set.rs-0492 */ }
/* FP:bit_set.rs-0493 */ 
/* FP:bit_set.rs-0494 */ // NOTE: The chunk size is computed on-the-fly on each manipulation of a chunk.
/* FP:bit_set.rs-0495 */ // This avoids storing it, as it's almost always CHUNK_BITS except for the last one.
/* FP:bit_set.rs-0496 */ #[derive(Clone, Debug, PartialEq, Eq)]
/* FP:bit_set.rs-0497 */ enum Chunk {
/* FP:bit_set.rs-0498 */     /// A chunk that is all zeros; we don't represent the zeros explicitly.
/* FP:bit_set.rs-0499 */     Zeros,
/* FP:bit_set.rs-0500 */ 
/* FP:bit_set.rs-0501 */     /// A chunk that is all ones; we don't represent the ones explicitly.
/* FP:bit_set.rs-0502 */     Ones,
/* FP:bit_set.rs-0503 */ 
/* FP:bit_set.rs-0504 */     /// A chunk that has a mix of zeros and ones, which are represented
/* FP:bit_set.rs-0505 */     /// explicitly and densely. It never has all zeros or all ones.
/* FP:bit_set.rs-0506 */     ///
/* FP:bit_set.rs-0507 */     /// If this is the final chunk there may be excess, unused words. This
/* FP:bit_set.rs-0508 */     /// turns out to be both simpler and have better performance than
/* FP:bit_set.rs-0509 */     /// allocating the minimum number of words, largely because we avoid having
/* FP:bit_set.rs-0510 */     /// to store the length, which would make this type larger. These excess
/* FP:bit_set.rs-0511 */     /// words are always zero, as are any excess bits in the final in-use word.
/* FP:bit_set.rs-0512 */     ///
/* FP:bit_set.rs-0513 */     /// The `ChunkSize` field is the count of 1s set in the chunk, and
/* FP:bit_set.rs-0514 */     /// must satisfy `0 < count < chunk_domain_size`.
/* FP:bit_set.rs-0515 */     ///
/* FP:bit_set.rs-0516 */     /// The words are within an `Rc` because it's surprisingly common to
/* FP:bit_set.rs-0517 */     /// duplicate an entire chunk, e.g. in `ChunkedBitSet::clone_from()`, or
/* FP:bit_set.rs-0518 */     /// when a `Mixed` chunk is union'd into a `Zeros` chunk. When we do need
/* FP:bit_set.rs-0519 */     /// to modify a chunk we use `Rc::make_mut`.
/* FP:bit_set.rs-0520 */     Mixed(ChunkSize, Rc<[Word; CHUNK_WORDS]>),
/* FP:bit_set.rs-0521 */ }
/* FP:bit_set.rs-0522 */ 
/* FP:bit_set.rs-0523 */ // This type is used a lot. Make sure it doesn't unintentionally get bigger.
/* FP:bit_set.rs-0524 */ #[cfg(target_pointer_width = "64")]
/* FP:bit_set.rs-0525 */ crate::static_assert_size!(Chunk, 16);
/* FP:bit_set.rs-0526 */ 
/* FP:bit_set.rs-0527 */ impl<T> ChunkedBitSet<T> {
/* FP:bit_set.rs-0528 */     pub fn domain_size(&self) -> usize {
/* FP:bit_set.rs-0529 */         self.domain_size
/* FP:bit_set.rs-0530 */     }
/* FP:bit_set.rs-0531 */ 
/* FP:bit_set.rs-0532 */     #[inline]
/* FP:bit_set.rs-0533 */     fn last_chunk_size(&self) -> ChunkSize {
/* FP:bit_set.rs-0534 */         let n = self.domain_size % CHUNK_BITS;
/* FP:bit_set.rs-0535 */         if n == 0 { CHUNK_BITS as ChunkSize } else { n as ChunkSize }
/* FP:bit_set.rs-0536 */     }
/* FP:bit_set.rs-0537 */ 
/* FP:bit_set.rs-0538 */     /// All the chunks have a chunk_domain_size of `CHUNK_BITS` except the final one.
/* FP:bit_set.rs-0539 */     #[inline]
/* FP:bit_set.rs-0540 */     fn chunk_domain_size(&self, chunk: usize) -> ChunkSize {
/* FP:bit_set.rs-0541 */         if chunk == self.chunks.len() - 1 {
/* FP:bit_set.rs-0542 */             self.last_chunk_size()
/* FP:bit_set.rs-0543 */         } else {
/* FP:bit_set.rs-0544 */             CHUNK_BITS as ChunkSize
/* FP:bit_set.rs-0545 */         }
/* FP:bit_set.rs-0546 */     }
/* FP:bit_set.rs-0547 */ 
/* FP:bit_set.rs-0548 */     #[cfg(test)]
/* FP:bit_set.rs-0549 */     fn assert_valid(&self) {
/* FP:bit_set.rs-0550 */         if self.domain_size == 0 {
/* FP:bit_set.rs-0551 */             assert!(self.chunks.is_empty());
/* FP:bit_set.rs-0552 */             return;
/* FP:bit_set.rs-0553 */         }
/* FP:bit_set.rs-0554 */ 
/* FP:bit_set.rs-0555 */         assert!((self.chunks.len() - 1) * CHUNK_BITS <= self.domain_size);
/* FP:bit_set.rs-0556 */         assert!(self.chunks.len() * CHUNK_BITS >= self.domain_size);
/* FP:bit_set.rs-0557 */         for (chunk_index, chunk) in self.chunks.iter().enumerate() {
/* FP:bit_set.rs-0558 */             let chunk_domain_size = self.chunk_domain_size(chunk_index);
/* FP:bit_set.rs-0559 */             chunk.assert_valid(chunk_domain_size);
/* FP:bit_set.rs-0560 */         }
/* FP:bit_set.rs-0561 */     }
/* FP:bit_set.rs-0562 */ }
/* FP:bit_set.rs-0563 */ 
/* FP:bit_set.rs-0564 */ impl<T: Idx> ChunkedBitSet<T> {
/* FP:bit_set.rs-0565 */     /// Creates a new bitset with a given `domain_size` and chunk kind.
/* FP:bit_set.rs-0566 */     fn new(domain_size: usize, is_empty: bool) -> Self {
/* FP:bit_set.rs-0567 */         let chunks = if domain_size == 0 {
/* FP:bit_set.rs-0568 */             Box::new([])
/* FP:bit_set.rs-0569 */         } else {
/* FP:bit_set.rs-0570 */             vec![if is_empty { Zeros } else { Ones }; num_chunks(domain_size)].into_boxed_slice()
/* FP:bit_set.rs-0571 */         };
/* FP:bit_set.rs-0572 */         ChunkedBitSet { domain_size, chunks, marker: PhantomData }
/* FP:bit_set.rs-0573 */     }
/* FP:bit_set.rs-0574 */ 
/* FP:bit_set.rs-0575 */     /// Creates a new, empty bitset with a given `domain_size`.
/* FP:bit_set.rs-0576 */     #[inline]
/* FP:bit_set.rs-0577 */     pub fn new_empty(domain_size: usize) -> Self {
/* FP:bit_set.rs-0578 */         ChunkedBitSet::new(domain_size, /* is_empty */ true)
/* FP:bit_set.rs-0579 */     }
/* FP:bit_set.rs-0580 */ 
/* FP:bit_set.rs-0581 */     /// Creates a new, filled bitset with a given `domain_size`.
/* FP:bit_set.rs-0582 */     #[inline]
/* FP:bit_set.rs-0583 */     pub fn new_filled(domain_size: usize) -> Self {
/* FP:bit_set.rs-0584 */         ChunkedBitSet::new(domain_size, /* is_empty */ false)
/* FP:bit_set.rs-0585 */     }
/* FP:bit_set.rs-0586 */ 
/* FP:bit_set.rs-0587 */     pub fn clear(&mut self) {
/* FP:bit_set.rs-0588 */         let domain_size = self.domain_size();
/* FP:bit_set.rs-0589 */         *self = ChunkedBitSet::new_empty(domain_size);
/* FP:bit_set.rs-0590 */     }
/* FP:bit_set.rs-0591 */ 
/* FP:bit_set.rs-0592 */     #[cfg(test)]
/* FP:bit_set.rs-0593 */     fn chunks(&self) -> &[Chunk] {
/* FP:bit_set.rs-0594 */         &self.chunks
/* FP:bit_set.rs-0595 */     }
/* FP:bit_set.rs-0596 */ 
/* FP:bit_set.rs-0597 */     /// Count the number of bits in the set.
/* FP:bit_set.rs-0598 */     pub fn count(&self) -> usize {
/* FP:bit_set.rs-0599 */         self.chunks
/* FP:bit_set.rs-0600 */             .iter()
/* FP:bit_set.rs-0601 */             .enumerate()
/* FP:bit_set.rs-0602 */             .map(|(index, chunk)| chunk.count(self.chunk_domain_size(index)))
/* FP:bit_set.rs-0603 */             .sum()
/* FP:bit_set.rs-0604 */     }
/* FP:bit_set.rs-0605 */ 
/* FP:bit_set.rs-0606 */     pub fn is_empty(&self) -> bool {
/* FP:bit_set.rs-0607 */         self.chunks.iter().all(|chunk| matches!(chunk, Zeros))
/* FP:bit_set.rs-0608 */     }
/* FP:bit_set.rs-0609 */ 
/* FP:bit_set.rs-0610 */     /// Returns `true` if `self` contains `elem`.
/* FP:bit_set.rs-0611 */     #[inline]
/* FP:bit_set.rs-0612 */     pub fn contains(&self, elem: T) -> bool {
/* FP:bit_set.rs-0613 */         assert!(elem.index() < self.domain_size);
/* FP:bit_set.rs-0614 */         let chunk = &self.chunks[chunk_index(elem)];
/* FP:bit_set.rs-0615 */         match &chunk {
/* FP:bit_set.rs-0616 */             Zeros => false,
/* FP:bit_set.rs-0617 */             Ones => true,
/* FP:bit_set.rs-0618 */             Mixed(_, words) => {
/* FP:bit_set.rs-0619 */                 let (word_index, mask) = chunk_word_index_and_mask(elem);
/* FP:bit_set.rs-0620 */                 (words[word_index] & mask) != 0
/* FP:bit_set.rs-0621 */             }
/* FP:bit_set.rs-0622 */         }
/* FP:bit_set.rs-0623 */     }
/* FP:bit_set.rs-0624 */ 
/* FP:bit_set.rs-0625 */     #[inline]
/* FP:bit_set.rs-0626 */     pub fn iter(&self) -> ChunkedBitIter<'_, T> {
/* FP:bit_set.rs-0627 */         ChunkedBitIter::new(self)
/* FP:bit_set.rs-0628 */     }
/* FP:bit_set.rs-0629 */ 
/* FP:bit_set.rs-0630 */     /// Insert `elem`. Returns whether the set has changed.
/* FP:bit_set.rs-0631 */     pub fn insert(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-0632 */         assert!(elem.index() < self.domain_size);
/* FP:bit_set.rs-0633 */         let chunk_index = chunk_index(elem);
/* FP:bit_set.rs-0634 */         let chunk_domain_size = self.chunk_domain_size(chunk_index);
/* FP:bit_set.rs-0635 */         let chunk = &mut self.chunks[chunk_index];
/* FP:bit_set.rs-0636 */         match *chunk {
/* FP:bit_set.rs-0637 */             Zeros => {
/* FP:bit_set.rs-0638 */                 if chunk_domain_size > 1 {
/* FP:bit_set.rs-0639 */                     #[cfg(feature = "nightly")]
/* FP:bit_set.rs-0640 */                     let mut words = {
/* FP:bit_set.rs-0641 */                         // We take some effort to avoid copying the words.
/* FP:bit_set.rs-0642 */                         let words = Rc::<[Word; CHUNK_WORDS]>::new_zeroed();
/* FP:bit_set.rs-0643 */                         // SAFETY: `words` can safely be all zeroes.
/* FP:bit_set.rs-0644 */                         unsafe { words.assume_init() }
/* FP:bit_set.rs-0645 */                     };
/* FP:bit_set.rs-0646 */                     #[cfg(not(feature = "nightly"))]
/* FP:bit_set.rs-0647 */                     let mut words = {
/* FP:bit_set.rs-0648 */                         // FIXME: unconditionally use `Rc::new_zeroed` once it is stable (#63291).
/* FP:bit_set.rs-0649 */                         let words = mem::MaybeUninit::<[Word; CHUNK_WORDS]>::zeroed();
/* FP:bit_set.rs-0650 */                         // SAFETY: `words` can safely be all zeroes.
/* FP:bit_set.rs-0651 */                         let words = unsafe { words.assume_init() };
/* FP:bit_set.rs-0652 */                         // Unfortunate possibly-large copy
/* FP:bit_set.rs-0653 */                         Rc::new(words)
/* FP:bit_set.rs-0654 */                     };
/* FP:bit_set.rs-0655 */                     let words_ref = Rc::get_mut(&mut words).unwrap();
/* FP:bit_set.rs-0656 */ 
/* FP:bit_set.rs-0657 */                     let (word_index, mask) = chunk_word_index_and_mask(elem);
/* FP:bit_set.rs-0658 */                     words_ref[word_index] |= mask;
/* FP:bit_set.rs-0659 */                     *chunk = Mixed(1, words);
/* FP:bit_set.rs-0660 */                 } else {
/* FP:bit_set.rs-0661 */                     *chunk = Ones;
/* FP:bit_set.rs-0662 */                 }
/* FP:bit_set.rs-0663 */                 true
/* FP:bit_set.rs-0664 */             }
/* FP:bit_set.rs-0665 */             Ones => false,
/* FP:bit_set.rs-0666 */             Mixed(ref mut count, ref mut words) => {
/* FP:bit_set.rs-0667 */                 // We skip all the work if the bit is already set.
/* FP:bit_set.rs-0668 */                 let (word_index, mask) = chunk_word_index_and_mask(elem);
/* FP:bit_set.rs-0669 */                 if (words[word_index] & mask) == 0 {
/* FP:bit_set.rs-0670 */                     *count += 1;
/* FP:bit_set.rs-0671 */                     if *count < chunk_domain_size {
/* FP:bit_set.rs-0672 */                         let words = Rc::make_mut(words);
/* FP:bit_set.rs-0673 */                         words[word_index] |= mask;
/* FP:bit_set.rs-0674 */                     } else {
/* FP:bit_set.rs-0675 */                         *chunk = Ones;
/* FP:bit_set.rs-0676 */                     }
/* FP:bit_set.rs-0677 */                     true
/* FP:bit_set.rs-0678 */                 } else {
/* FP:bit_set.rs-0679 */                     false
/* FP:bit_set.rs-0680 */                 }
/* FP:bit_set.rs-0681 */             }
/* FP:bit_set.rs-0682 */         }
/* FP:bit_set.rs-0683 */     }
/* FP:bit_set.rs-0684 */ 
/* FP:bit_set.rs-0685 */     /// Sets all bits to true.
/* FP:bit_set.rs-0686 */     pub fn insert_all(&mut self) {
/* FP:bit_set.rs-0687 */         for chunk in self.chunks.iter_mut() {
/* FP:bit_set.rs-0688 */             *chunk = Ones;
/* FP:bit_set.rs-0689 */         }
/* FP:bit_set.rs-0690 */     }
/* FP:bit_set.rs-0691 */ 
/* FP:bit_set.rs-0692 */     /// Returns `true` if the set has changed.
/* FP:bit_set.rs-0693 */     pub fn remove(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-0694 */         assert!(elem.index() < self.domain_size);
/* FP:bit_set.rs-0695 */         let chunk_index = chunk_index(elem);
/* FP:bit_set.rs-0696 */         let chunk_domain_size = self.chunk_domain_size(chunk_index);
/* FP:bit_set.rs-0697 */         let chunk = &mut self.chunks[chunk_index];
/* FP:bit_set.rs-0698 */         match *chunk {
/* FP:bit_set.rs-0699 */             Zeros => false,
/* FP:bit_set.rs-0700 */             Ones => {
/* FP:bit_set.rs-0701 */                 if chunk_domain_size > 1 {
/* FP:bit_set.rs-0702 */                     #[cfg(feature = "nightly")]
/* FP:bit_set.rs-0703 */                     let mut words = {
/* FP:bit_set.rs-0704 */                         // We take some effort to avoid copying the words.
/* FP:bit_set.rs-0705 */                         let words = Rc::<[Word; CHUNK_WORDS]>::new_zeroed();
/* FP:bit_set.rs-0706 */                         // SAFETY: `words` can safely be all zeroes.
/* FP:bit_set.rs-0707 */                         unsafe { words.assume_init() }
/* FP:bit_set.rs-0708 */                     };
/* FP:bit_set.rs-0709 */                     #[cfg(not(feature = "nightly"))]
/* FP:bit_set.rs-0710 */                     let mut words = {
/* FP:bit_set.rs-0711 */                         // FIXME: unconditionally use `Rc::new_zeroed` once it is stable (#63291).
/* FP:bit_set.rs-0712 */                         let words = mem::MaybeUninit::<[Word; CHUNK_WORDS]>::zeroed();
/* FP:bit_set.rs-0713 */                         // SAFETY: `words` can safely be all zeroes.
/* FP:bit_set.rs-0714 */                         let words = unsafe { words.assume_init() };
/* FP:bit_set.rs-0715 */                         // Unfortunate possibly-large copy
/* FP:bit_set.rs-0716 */                         Rc::new(words)
/* FP:bit_set.rs-0717 */                     };
/* FP:bit_set.rs-0718 */                     let words_ref = Rc::get_mut(&mut words).unwrap();
/* FP:bit_set.rs-0719 */ 
/* FP:bit_set.rs-0720 */                     // Set only the bits in use.
/* FP:bit_set.rs-0721 */                     let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0722 */                     words_ref[..num_words].fill(!0);
/* FP:bit_set.rs-0723 */                     clear_excess_bits_in_final_word(
/* FP:bit_set.rs-0724 */                         chunk_domain_size as usize,
/* FP:bit_set.rs-0725 */                         &mut words_ref[..num_words],
/* FP:bit_set.rs-0726 */                     );
/* FP:bit_set.rs-0727 */                     let (word_index, mask) = chunk_word_index_and_mask(elem);
/* FP:bit_set.rs-0728 */                     words_ref[word_index] &= !mask;
/* FP:bit_set.rs-0729 */                     *chunk = Mixed(chunk_domain_size - 1, words);
/* FP:bit_set.rs-0730 */                 } else {
/* FP:bit_set.rs-0731 */                     *chunk = Zeros;
/* FP:bit_set.rs-0732 */                 }
/* FP:bit_set.rs-0733 */                 true
/* FP:bit_set.rs-0734 */             }
/* FP:bit_set.rs-0735 */             Mixed(ref mut count, ref mut words) => {
/* FP:bit_set.rs-0736 */                 // We skip all the work if the bit is already clear.
/* FP:bit_set.rs-0737 */                 let (word_index, mask) = chunk_word_index_and_mask(elem);
/* FP:bit_set.rs-0738 */                 if (words[word_index] & mask) != 0 {
/* FP:bit_set.rs-0739 */                     *count -= 1;
/* FP:bit_set.rs-0740 */                     if *count > 0 {
/* FP:bit_set.rs-0741 */                         let words = Rc::make_mut(words);
/* FP:bit_set.rs-0742 */                         words[word_index] &= !mask;
/* FP:bit_set.rs-0743 */                     } else {
/* FP:bit_set.rs-0744 */                         *chunk = Zeros
/* FP:bit_set.rs-0745 */                     }
/* FP:bit_set.rs-0746 */                     true
/* FP:bit_set.rs-0747 */                 } else {
/* FP:bit_set.rs-0748 */                     false
/* FP:bit_set.rs-0749 */                 }
/* FP:bit_set.rs-0750 */             }
/* FP:bit_set.rs-0751 */         }
/* FP:bit_set.rs-0752 */     }
/* FP:bit_set.rs-0753 */ 
/* FP:bit_set.rs-0754 */     fn chunk_iter(&self, chunk_index: usize) -> ChunkIter<'_> {
/* FP:bit_set.rs-0755 */         let chunk_domain_size = self.chunk_domain_size(chunk_index);
/* FP:bit_set.rs-0756 */         match self.chunks.get(chunk_index) {
/* FP:bit_set.rs-0757 */             Some(Zeros) => ChunkIter::Zeros,
/* FP:bit_set.rs-0758 */             Some(Ones) => ChunkIter::Ones(0..chunk_domain_size as usize),
/* FP:bit_set.rs-0759 */             Some(Mixed(_, words)) => {
/* FP:bit_set.rs-0760 */                 let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0761 */                 ChunkIter::Mixed(BitIter::new(&words[0..num_words]))
/* FP:bit_set.rs-0762 */             }
/* FP:bit_set.rs-0763 */             None => ChunkIter::Finished,
/* FP:bit_set.rs-0764 */         }
/* FP:bit_set.rs-0765 */     }
/* FP:bit_set.rs-0766 */ 
/* FP:bit_set.rs-0767 */     bit_relations_inherent_impls! {}
/* FP:bit_set.rs-0768 */ }
/* FP:bit_set.rs-0769 */ 
/* FP:bit_set.rs-0770 */ impl<T: Idx> BitRelations<ChunkedBitSet<T>> for ChunkedBitSet<T> {
/* FP:bit_set.rs-0771 */     fn union(&mut self, other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0772 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0773 */ 
/* FP:bit_set.rs-0774 */         let num_chunks = self.chunks.len();
/* FP:bit_set.rs-0775 */         debug_assert_eq!(num_chunks, other.chunks.len());
/* FP:bit_set.rs-0776 */ 
/* FP:bit_set.rs-0777 */         let last_chunk_size = self.last_chunk_size();
/* FP:bit_set.rs-0778 */         debug_assert_eq!(last_chunk_size, other.last_chunk_size());
/* FP:bit_set.rs-0779 */ 
/* FP:bit_set.rs-0780 */         let mut changed = false;
/* FP:bit_set.rs-0781 */         for (chunk_index, (mut self_chunk, other_chunk)) in
/* FP:bit_set.rs-0782 */             self.chunks.iter_mut().zip(other.chunks.iter()).enumerate()
/* FP:bit_set.rs-0783 */         {
/* FP:bit_set.rs-0784 */             let chunk_domain_size = if chunk_index + 1 == num_chunks {
/* FP:bit_set.rs-0785 */                 last_chunk_size
/* FP:bit_set.rs-0786 */             } else {
/* FP:bit_set.rs-0787 */                 CHUNK_BITS as ChunkSize
/* FP:bit_set.rs-0788 */             };
/* FP:bit_set.rs-0789 */ 
/* FP:bit_set.rs-0790 */             match (&mut self_chunk, &other_chunk) {
/* FP:bit_set.rs-0791 */                 (_, Zeros) | (Ones, _) => {}
/* FP:bit_set.rs-0792 */                 (Zeros, Ones) | (Mixed(..), Ones) | (Zeros, Mixed(..)) => {
/* FP:bit_set.rs-0793 */                     // `other_chunk` fully overwrites `self_chunk`
/* FP:bit_set.rs-0794 */                     *self_chunk = other_chunk.clone();
/* FP:bit_set.rs-0795 */                     changed = true;
/* FP:bit_set.rs-0796 */                 }
/* FP:bit_set.rs-0797 */                 (
/* FP:bit_set.rs-0798 */                     Mixed(self_chunk_count, self_chunk_words),
/* FP:bit_set.rs-0799 */                     Mixed(_other_chunk_count, other_chunk_words),
/* FP:bit_set.rs-0800 */                 ) => {
/* FP:bit_set.rs-0801 */                     // First check if the operation would change
/* FP:bit_set.rs-0802 */                     // `self_chunk.words`. If not, we can avoid allocating some
/* FP:bit_set.rs-0803 */                     // words, and this happens often enough that it's a
/* FP:bit_set.rs-0804 */                     // performance win. Also, we only need to operate on the
/* FP:bit_set.rs-0805 */                     // in-use words, hence the slicing.
/* FP:bit_set.rs-0806 */                     let op = |a, b| a | b;
/* FP:bit_set.rs-0807 */                     let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0808 */                     if bitwise_changes(
/* FP:bit_set.rs-0809 */                         &self_chunk_words[0..num_words],
/* FP:bit_set.rs-0810 */                         &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0811 */                         op,
/* FP:bit_set.rs-0812 */                     ) {
/* FP:bit_set.rs-0813 */                         let self_chunk_words = Rc::make_mut(self_chunk_words);
/* FP:bit_set.rs-0814 */                         let has_changed = bitwise(
/* FP:bit_set.rs-0815 */                             &mut self_chunk_words[0..num_words],
/* FP:bit_set.rs-0816 */                             &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0817 */                             op,
/* FP:bit_set.rs-0818 */                         );
/* FP:bit_set.rs-0819 */                         debug_assert!(has_changed);
/* FP:bit_set.rs-0820 */                         *self_chunk_count = self_chunk_words[0..num_words]
/* FP:bit_set.rs-0821 */                             .iter()
/* FP:bit_set.rs-0822 */                             .map(|w| w.count_ones() as ChunkSize)
/* FP:bit_set.rs-0823 */                             .sum();
/* FP:bit_set.rs-0824 */                         if *self_chunk_count == chunk_domain_size {
/* FP:bit_set.rs-0825 */                             *self_chunk = Ones;
/* FP:bit_set.rs-0826 */                         }
/* FP:bit_set.rs-0827 */                         changed = true;
/* FP:bit_set.rs-0828 */                     }
/* FP:bit_set.rs-0829 */                 }
/* FP:bit_set.rs-0830 */             }
/* FP:bit_set.rs-0831 */         }
/* FP:bit_set.rs-0832 */         changed
/* FP:bit_set.rs-0833 */     }
/* FP:bit_set.rs-0834 */ 
/* FP:bit_set.rs-0835 */     fn subtract(&mut self, other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0836 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0837 */ 
/* FP:bit_set.rs-0838 */         let num_chunks = self.chunks.len();
/* FP:bit_set.rs-0839 */         debug_assert_eq!(num_chunks, other.chunks.len());
/* FP:bit_set.rs-0840 */ 
/* FP:bit_set.rs-0841 */         let last_chunk_size = self.last_chunk_size();
/* FP:bit_set.rs-0842 */         debug_assert_eq!(last_chunk_size, other.last_chunk_size());
/* FP:bit_set.rs-0843 */ 
/* FP:bit_set.rs-0844 */         let mut changed = false;
/* FP:bit_set.rs-0845 */         for (chunk_index, (mut self_chunk, other_chunk)) in
/* FP:bit_set.rs-0846 */             self.chunks.iter_mut().zip(other.chunks.iter()).enumerate()
/* FP:bit_set.rs-0847 */         {
/* FP:bit_set.rs-0848 */             let chunk_domain_size = if chunk_index + 1 == num_chunks {
/* FP:bit_set.rs-0849 */                 last_chunk_size
/* FP:bit_set.rs-0850 */             } else {
/* FP:bit_set.rs-0851 */                 CHUNK_BITS as ChunkSize
/* FP:bit_set.rs-0852 */             };
/* FP:bit_set.rs-0853 */ 
/* FP:bit_set.rs-0854 */             match (&mut self_chunk, &other_chunk) {
/* FP:bit_set.rs-0855 */                 (Zeros, _) | (_, Zeros) => {}
/* FP:bit_set.rs-0856 */                 (Ones | Mixed(_, _), Ones) => {
/* FP:bit_set.rs-0857 */                     changed = true;
/* FP:bit_set.rs-0858 */                     *self_chunk = Zeros;
/* FP:bit_set.rs-0859 */                 }
/* FP:bit_set.rs-0860 */                 (Ones, Mixed(other_chunk_count, other_chunk_words)) => {
/* FP:bit_set.rs-0861 */                     changed = true;
/* FP:bit_set.rs-0862 */                     let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0863 */                     debug_assert!(num_words > 0 && num_words <= CHUNK_WORDS);
/* FP:bit_set.rs-0864 */                     let mut tail_mask =
/* FP:bit_set.rs-0865 */                         1 << (chunk_domain_size - ((num_words - 1) * WORD_BITS) as u16) - 1;
/* FP:bit_set.rs-0866 */                     let mut self_chunk_words = **other_chunk_words;
/* FP:bit_set.rs-0867 */                     for word in self_chunk_words[0..num_words].iter_mut().rev() {
/* FP:bit_set.rs-0868 */                         *word = !*word & tail_mask;
/* FP:bit_set.rs-0869 */                         tail_mask = u64::MAX;
/* FP:bit_set.rs-0870 */                     }
/* FP:bit_set.rs-0871 */                     let self_chunk_count = chunk_domain_size - *other_chunk_count;
/* FP:bit_set.rs-0872 */                     debug_assert_eq!(
/* FP:bit_set.rs-0873 */                         self_chunk_count,
/* FP:bit_set.rs-0874 */                         self_chunk_words[0..num_words]
/* FP:bit_set.rs-0875 */                             .iter()
/* FP:bit_set.rs-0876 */                             .map(|w| w.count_ones() as ChunkSize)
/* FP:bit_set.rs-0877 */                             .sum()
/* FP:bit_set.rs-0878 */                     );
/* FP:bit_set.rs-0879 */                     *self_chunk = Mixed(self_chunk_count, Rc::new(self_chunk_words));
/* FP:bit_set.rs-0880 */                 }
/* FP:bit_set.rs-0881 */                 (
/* FP:bit_set.rs-0882 */                     Mixed(self_chunk_count, self_chunk_words),
/* FP:bit_set.rs-0883 */                     Mixed(_other_chunk_count, other_chunk_words),
/* FP:bit_set.rs-0884 */                 ) => {
/* FP:bit_set.rs-0885 */                     // See [`<Self as BitRelations<ChunkedBitSet<T>>>::union`] for the explanation
/* FP:bit_set.rs-0886 */                     let op = |a: u64, b: u64| a & !b;
/* FP:bit_set.rs-0887 */                     let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0888 */                     if bitwise_changes(
/* FP:bit_set.rs-0889 */                         &self_chunk_words[0..num_words],
/* FP:bit_set.rs-0890 */                         &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0891 */                         op,
/* FP:bit_set.rs-0892 */                     ) {
/* FP:bit_set.rs-0893 */                         let self_chunk_words = Rc::make_mut(self_chunk_words);
/* FP:bit_set.rs-0894 */                         let has_changed = bitwise(
/* FP:bit_set.rs-0895 */                             &mut self_chunk_words[0..num_words],
/* FP:bit_set.rs-0896 */                             &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0897 */                             op,
/* FP:bit_set.rs-0898 */                         );
/* FP:bit_set.rs-0899 */                         debug_assert!(has_changed);
/* FP:bit_set.rs-0900 */                         *self_chunk_count = self_chunk_words[0..num_words]
/* FP:bit_set.rs-0901 */                             .iter()
/* FP:bit_set.rs-0902 */                             .map(|w| w.count_ones() as ChunkSize)
/* FP:bit_set.rs-0903 */                             .sum();
/* FP:bit_set.rs-0904 */                         if *self_chunk_count == 0 {
/* FP:bit_set.rs-0905 */                             *self_chunk = Zeros;
/* FP:bit_set.rs-0906 */                         }
/* FP:bit_set.rs-0907 */                         changed = true;
/* FP:bit_set.rs-0908 */                     }
/* FP:bit_set.rs-0909 */                 }
/* FP:bit_set.rs-0910 */             }
/* FP:bit_set.rs-0911 */         }
/* FP:bit_set.rs-0912 */         changed
/* FP:bit_set.rs-0913 */     }
/* FP:bit_set.rs-0914 */ 
/* FP:bit_set.rs-0915 */     fn intersect(&mut self, other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0916 */         assert_eq!(self.domain_size, other.domain_size);
/* FP:bit_set.rs-0917 */ 
/* FP:bit_set.rs-0918 */         let num_chunks = self.chunks.len();
/* FP:bit_set.rs-0919 */         debug_assert_eq!(num_chunks, other.chunks.len());
/* FP:bit_set.rs-0920 */ 
/* FP:bit_set.rs-0921 */         let last_chunk_size = self.last_chunk_size();
/* FP:bit_set.rs-0922 */         debug_assert_eq!(last_chunk_size, other.last_chunk_size());
/* FP:bit_set.rs-0923 */ 
/* FP:bit_set.rs-0924 */         let mut changed = false;
/* FP:bit_set.rs-0925 */         for (chunk_index, (mut self_chunk, other_chunk)) in
/* FP:bit_set.rs-0926 */             self.chunks.iter_mut().zip(other.chunks.iter()).enumerate()
/* FP:bit_set.rs-0927 */         {
/* FP:bit_set.rs-0928 */             let chunk_domain_size = if chunk_index + 1 == num_chunks {
/* FP:bit_set.rs-0929 */                 last_chunk_size
/* FP:bit_set.rs-0930 */             } else {
/* FP:bit_set.rs-0931 */                 CHUNK_BITS as ChunkSize
/* FP:bit_set.rs-0932 */             };
/* FP:bit_set.rs-0933 */ 
/* FP:bit_set.rs-0934 */             match (&mut self_chunk, &other_chunk) {
/* FP:bit_set.rs-0935 */                 (Zeros, _) | (_, Ones) => {}
/* FP:bit_set.rs-0936 */                 (Ones, Zeros | Mixed(..)) | (Mixed(..), Zeros) => {
/* FP:bit_set.rs-0937 */                     changed = true;
/* FP:bit_set.rs-0938 */                     *self_chunk = other_chunk.clone();
/* FP:bit_set.rs-0939 */                 }
/* FP:bit_set.rs-0940 */                 (
/* FP:bit_set.rs-0941 */                     Mixed(self_chunk_count, self_chunk_words),
/* FP:bit_set.rs-0942 */                     Mixed(_other_chunk_count, other_chunk_words),
/* FP:bit_set.rs-0943 */                 ) => {
/* FP:bit_set.rs-0944 */                     // See [`<Self as BitRelations<ChunkedBitSet<T>>>::union`] for the explanation
/* FP:bit_set.rs-0945 */                     let op = |a, b| a & b;
/* FP:bit_set.rs-0946 */                     let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-0947 */                     if bitwise_changes(
/* FP:bit_set.rs-0948 */                         &self_chunk_words[0..num_words],
/* FP:bit_set.rs-0949 */                         &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0950 */                         op,
/* FP:bit_set.rs-0951 */                     ) {
/* FP:bit_set.rs-0952 */                         let self_chunk_words = Rc::make_mut(self_chunk_words);
/* FP:bit_set.rs-0953 */                         let has_changed = bitwise(
/* FP:bit_set.rs-0954 */                             &mut self_chunk_words[0..num_words],
/* FP:bit_set.rs-0955 */                             &other_chunk_words[0..num_words],
/* FP:bit_set.rs-0956 */                             op,
/* FP:bit_set.rs-0957 */                         );
/* FP:bit_set.rs-0958 */                         debug_assert!(has_changed);
/* FP:bit_set.rs-0959 */                         *self_chunk_count = self_chunk_words[0..num_words]
/* FP:bit_set.rs-0960 */                             .iter()
/* FP:bit_set.rs-0961 */                             .map(|w| w.count_ones() as ChunkSize)
/* FP:bit_set.rs-0962 */                             .sum();
/* FP:bit_set.rs-0963 */                         if *self_chunk_count == 0 {
/* FP:bit_set.rs-0964 */                             *self_chunk = Zeros;
/* FP:bit_set.rs-0965 */                         }
/* FP:bit_set.rs-0966 */                         changed = true;
/* FP:bit_set.rs-0967 */                     }
/* FP:bit_set.rs-0968 */                 }
/* FP:bit_set.rs-0969 */             }
/* FP:bit_set.rs-0970 */         }
/* FP:bit_set.rs-0971 */ 
/* FP:bit_set.rs-0972 */         changed
/* FP:bit_set.rs-0973 */     }
/* FP:bit_set.rs-0974 */ }
/* FP:bit_set.rs-0975 */ 
/* FP:bit_set.rs-0976 */ impl<T: Idx> BitRelations<ChunkedBitSet<T>> for DenseBitSet<T> {
/* FP:bit_set.rs-0977 */     fn union(&mut self, other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0978 */         sequential_update(|elem| self.insert(elem), other.iter())
/* FP:bit_set.rs-0979 */     }
/* FP:bit_set.rs-0980 */ 
/* FP:bit_set.rs-0981 */     fn subtract(&mut self, _other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0982 */         unimplemented!("implement if/when necessary");
/* FP:bit_set.rs-0983 */     }
/* FP:bit_set.rs-0984 */ 
/* FP:bit_set.rs-0985 */     fn intersect(&mut self, other: &ChunkedBitSet<T>) -> bool {
/* FP:bit_set.rs-0986 */         assert_eq!(self.domain_size(), other.domain_size);
/* FP:bit_set.rs-0987 */         let mut changed = false;
/* FP:bit_set.rs-0988 */         for (i, chunk) in other.chunks.iter().enumerate() {
/* FP:bit_set.rs-0989 */             let mut words = &mut self.words[i * CHUNK_WORDS..];
/* FP:bit_set.rs-0990 */             if words.len() > CHUNK_WORDS {
/* FP:bit_set.rs-0991 */                 words = &mut words[..CHUNK_WORDS];
/* FP:bit_set.rs-0992 */             }
/* FP:bit_set.rs-0993 */             match chunk {
/* FP:bit_set.rs-0994 */                 Zeros => {
/* FP:bit_set.rs-0995 */                     for word in words {
/* FP:bit_set.rs-0996 */                         if *word != 0 {
/* FP:bit_set.rs-0997 */                             changed = true;
/* FP:bit_set.rs-0998 */                             *word = 0;
/* FP:bit_set.rs-0999 */                         }
/* FP:bit_set.rs-1000 */                     }
/* FP:bit_set.rs-1001 */                 }
/* FP:bit_set.rs-1002 */                 Ones => (),
/* FP:bit_set.rs-1003 */                 Mixed(_, data) => {
/* FP:bit_set.rs-1004 */                     for (i, word) in words.iter_mut().enumerate() {
/* FP:bit_set.rs-1005 */                         let new_val = *word & data[i];
/* FP:bit_set.rs-1006 */                         if new_val != *word {
/* FP:bit_set.rs-1007 */                             changed = true;
/* FP:bit_set.rs-1008 */                             *word = new_val;
/* FP:bit_set.rs-1009 */                         }
/* FP:bit_set.rs-1010 */                     }
/* FP:bit_set.rs-1011 */                 }
/* FP:bit_set.rs-1012 */             }
/* FP:bit_set.rs-1013 */         }
/* FP:bit_set.rs-1014 */         changed
/* FP:bit_set.rs-1015 */     }
/* FP:bit_set.rs-1016 */ }
/* FP:bit_set.rs-1017 */ 
/* FP:bit_set.rs-1018 */ impl<T> Clone for ChunkedBitSet<T> {
/* FP:bit_set.rs-1019 */     fn clone(&self) -> Self {
/* FP:bit_set.rs-1020 */         ChunkedBitSet {
/* FP:bit_set.rs-1021 */             domain_size: self.domain_size,
/* FP:bit_set.rs-1022 */             chunks: self.chunks.clone(),
/* FP:bit_set.rs-1023 */             marker: PhantomData,
/* FP:bit_set.rs-1024 */         }
/* FP:bit_set.rs-1025 */     }
/* FP:bit_set.rs-1026 */ 
/* FP:bit_set.rs-1027 */     /// WARNING: this implementation of clone_from will panic if the two
/* FP:bit_set.rs-1028 */     /// bitsets have different domain sizes. This constraint is not inherent to
/* FP:bit_set.rs-1029 */     /// `clone_from`, but it works with the existing call sites and allows a
/* FP:bit_set.rs-1030 */     /// faster implementation, which is important because this function is hot.
/* FP:bit_set.rs-1031 */     fn clone_from(&mut self, from: &Self) {
/* FP:bit_set.rs-1032 */         assert_eq!(self.domain_size, from.domain_size);
/* FP:bit_set.rs-1033 */         debug_assert_eq!(self.chunks.len(), from.chunks.len());
/* FP:bit_set.rs-1034 */ 
/* FP:bit_set.rs-1035 */         self.chunks.clone_from(&from.chunks)
/* FP:bit_set.rs-1036 */     }
/* FP:bit_set.rs-1037 */ }
/* FP:bit_set.rs-1038 */ 
/* FP:bit_set.rs-1039 */ pub struct ChunkedBitIter<'a, T: Idx> {
/* FP:bit_set.rs-1040 */     bit_set: &'a ChunkedBitSet<T>,
/* FP:bit_set.rs-1041 */ 
/* FP:bit_set.rs-1042 */     // The index of the current chunk.
/* FP:bit_set.rs-1043 */     chunk_index: usize,
/* FP:bit_set.rs-1044 */ 
/* FP:bit_set.rs-1045 */     // The sub-iterator for the current chunk.
/* FP:bit_set.rs-1046 */     chunk_iter: ChunkIter<'a>,
/* FP:bit_set.rs-1047 */ }
/* FP:bit_set.rs-1048 */ 
/* FP:bit_set.rs-1049 */ impl<'a, T: Idx> ChunkedBitIter<'a, T> {
/* FP:bit_set.rs-1050 */     #[inline]
/* FP:bit_set.rs-1051 */     fn new(bit_set: &'a ChunkedBitSet<T>) -> ChunkedBitIter<'a, T> {
/* FP:bit_set.rs-1052 */         ChunkedBitIter { bit_set, chunk_index: 0, chunk_iter: bit_set.chunk_iter(0) }
/* FP:bit_set.rs-1053 */     }
/* FP:bit_set.rs-1054 */ }
/* FP:bit_set.rs-1055 */ 
/* FP:bit_set.rs-1056 */ impl<'a, T: Idx> Iterator for ChunkedBitIter<'a, T> {
/* FP:bit_set.rs-1057 */     type Item = T;
/* FP:bit_set.rs-1058 */ 
/* FP:bit_set.rs-1059 */     fn next(&mut self) -> Option<T> {
/* FP:bit_set.rs-1060 */         loop {
/* FP:bit_set.rs-1061 */             match &mut self.chunk_iter {
/* FP:bit_set.rs-1062 */                 ChunkIter::Zeros => {}
/* FP:bit_set.rs-1063 */                 ChunkIter::Ones(iter) => {
/* FP:bit_set.rs-1064 */                     if let Some(next) = iter.next() {
/* FP:bit_set.rs-1065 */                         return Some(T::new(next + self.chunk_index * CHUNK_BITS));
/* FP:bit_set.rs-1066 */                     }
/* FP:bit_set.rs-1067 */                 }
/* FP:bit_set.rs-1068 */                 ChunkIter::Mixed(iter) => {
/* FP:bit_set.rs-1069 */                     if let Some(next) = iter.next() {
/* FP:bit_set.rs-1070 */                         return Some(T::new(next + self.chunk_index * CHUNK_BITS));
/* FP:bit_set.rs-1071 */                     }
/* FP:bit_set.rs-1072 */                 }
/* FP:bit_set.rs-1073 */                 ChunkIter::Finished => return None,
/* FP:bit_set.rs-1074 */             }
/* FP:bit_set.rs-1075 */             self.chunk_index += 1;
/* FP:bit_set.rs-1076 */             self.chunk_iter = self.bit_set.chunk_iter(self.chunk_index);
/* FP:bit_set.rs-1077 */         }
/* FP:bit_set.rs-1078 */     }
/* FP:bit_set.rs-1079 */ }
/* FP:bit_set.rs-1080 */ 
/* FP:bit_set.rs-1081 */ impl Chunk {
/* FP:bit_set.rs-1082 */     #[cfg(test)]
/* FP:bit_set.rs-1083 */     fn assert_valid(&self, chunk_domain_size: ChunkSize) {
/* FP:bit_set.rs-1084 */         assert!(chunk_domain_size as usize <= CHUNK_BITS);
/* FP:bit_set.rs-1085 */         match *self {
/* FP:bit_set.rs-1086 */             Zeros | Ones => {}
/* FP:bit_set.rs-1087 */             Mixed(count, ref words) => {
/* FP:bit_set.rs-1088 */                 assert!(0 < count && count < chunk_domain_size);
/* FP:bit_set.rs-1089 */ 
/* FP:bit_set.rs-1090 */                 // Check the number of set bits matches `count`.
/* FP:bit_set.rs-1091 */                 assert_eq!(
/* FP:bit_set.rs-1092 */                     words.iter().map(|w| w.count_ones() as ChunkSize).sum::<ChunkSize>(),
/* FP:bit_set.rs-1093 */                     count
/* FP:bit_set.rs-1094 */                 );
/* FP:bit_set.rs-1095 */ 
/* FP:bit_set.rs-1096 */                 // Check the not-in-use words are all zeroed.
/* FP:bit_set.rs-1097 */                 let num_words = num_words(chunk_domain_size as usize);
/* FP:bit_set.rs-1098 */                 if num_words < CHUNK_WORDS {
/* FP:bit_set.rs-1099 */                     assert_eq!(
/* FP:bit_set.rs-1100 */                         words[num_words..]
/* FP:bit_set.rs-1101 */                             .iter()
/* FP:bit_set.rs-1102 */                             .map(|w| w.count_ones() as ChunkSize)
/* FP:bit_set.rs-1103 */                             .sum::<ChunkSize>(),
/* FP:bit_set.rs-1104 */                         0
/* FP:bit_set.rs-1105 */                     );
/* FP:bit_set.rs-1106 */                 }
/* FP:bit_set.rs-1107 */             }
/* FP:bit_set.rs-1108 */         }
/* FP:bit_set.rs-1109 */     }
/* FP:bit_set.rs-1110 */ 
/* FP:bit_set.rs-1111 */     /// Count the number of 1s in the chunk.
/* FP:bit_set.rs-1112 */     fn count(&self, chunk_domain_size: ChunkSize) -> usize {
/* FP:bit_set.rs-1113 */         match *self {
/* FP:bit_set.rs-1114 */             Zeros => 0,
/* FP:bit_set.rs-1115 */             Ones => chunk_domain_size as usize,
/* FP:bit_set.rs-1116 */             Mixed(count, _) => count as usize,
/* FP:bit_set.rs-1117 */         }
/* FP:bit_set.rs-1118 */     }
/* FP:bit_set.rs-1119 */ }
/* FP:bit_set.rs-1120 */ 
/* FP:bit_set.rs-1121 */ enum ChunkIter<'a> {
/* FP:bit_set.rs-1122 */     Zeros,
/* FP:bit_set.rs-1123 */     Ones(Range<usize>),
/* FP:bit_set.rs-1124 */     Mixed(BitIter<'a, usize>),
/* FP:bit_set.rs-1125 */     Finished,
/* FP:bit_set.rs-1126 */ }
/* FP:bit_set.rs-1127 */ 
/* FP:bit_set.rs-1128 */ // Applies a function to mutate a bitset, and returns true if any
/* FP:bit_set.rs-1129 */ // of the applications return true
/* FP:bit_set.rs-1130 */ fn sequential_update<T: Idx>(
/* FP:bit_set.rs-1131 */     mut self_update: impl FnMut(T) -> bool,
/* FP:bit_set.rs-1132 */     it: impl Iterator<Item = T>,
/* FP:bit_set.rs-1133 */ ) -> bool {
/* FP:bit_set.rs-1134 */     it.fold(false, |changed, elem| self_update(elem) | changed)
/* FP:bit_set.rs-1135 */ }
/* FP:bit_set.rs-1136 */ 
/* FP:bit_set.rs-1137 */ impl<T: Idx> fmt::Debug for ChunkedBitSet<T> {
/* FP:bit_set.rs-1138 */     fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-1139 */         w.debug_list().entries(self.iter()).finish()
/* FP:bit_set.rs-1140 */     }
/* FP:bit_set.rs-1141 */ }
/* FP:bit_set.rs-1142 */ 
/* FP:bit_set.rs-1143 */ /// Sets `out_vec[i] = op(out_vec[i], in_vec[i])` for each index `i` in both
/* FP:bit_set.rs-1144 */ /// slices. The slices must have the same length.
/* FP:bit_set.rs-1145 */ ///
/* FP:bit_set.rs-1146 */ /// Returns true if at least one bit in `out_vec` was changed.
/* FP:bit_set.rs-1147 */ ///
/* FP:bit_set.rs-1148 */ /// ## Warning
/* FP:bit_set.rs-1149 */ /// Some bitwise operations (e.g. union-not, xor) can set output bits that were
/* FP:bit_set.rs-1150 */ /// unset in in both inputs. If this happens in the last word/chunk of a bitset,
/* FP:bit_set.rs-1151 */ /// it can cause the bitset to contain out-of-domain values, which need to
/* FP:bit_set.rs-1152 */ /// be cleared with `clear_excess_bits_in_final_word`. This also makes the
/* FP:bit_set.rs-1153 */ /// "changed" return value unreliable, because the change might have only
/* FP:bit_set.rs-1154 */ /// affected excess bits.
/* FP:bit_set.rs-1155 */ #[inline]
/* FP:bit_set.rs-1156 */ fn bitwise<Op>(out_vec: &mut [Word], in_vec: &[Word], op: Op) -> bool
/* FP:bit_set.rs-1157 */ where
/* FP:bit_set.rs-1158 */     Op: Fn(Word, Word) -> Word,
/* FP:bit_set.rs-1159 */ {
/* FP:bit_set.rs-1160 */     assert_eq!(out_vec.len(), in_vec.len());
/* FP:bit_set.rs-1161 */     let mut changed = 0;
/* FP:bit_set.rs-1162 */     for (out_elem, in_elem) in iter::zip(out_vec, in_vec) {
/* FP:bit_set.rs-1163 */         let old_val = *out_elem;
/* FP:bit_set.rs-1164 */         let new_val = op(old_val, *in_elem);
/* FP:bit_set.rs-1165 */         *out_elem = new_val;
/* FP:bit_set.rs-1166 */         // This is essentially equivalent to a != with changed being a bool, but
/* FP:bit_set.rs-1167 */         // in practice this code gets auto-vectorized by the compiler for most
/* FP:bit_set.rs-1168 */         // operators. Using != here causes us to generate quite poor code as the
/* FP:bit_set.rs-1169 */         // compiler tries to go back to a boolean on each loop iteration.
/* FP:bit_set.rs-1170 */         changed |= old_val ^ new_val;
/* FP:bit_set.rs-1171 */     }
/* FP:bit_set.rs-1172 */     changed != 0
/* FP:bit_set.rs-1173 */ }
/* FP:bit_set.rs-1174 */ 
/* FP:bit_set.rs-1175 */ /// Does this bitwise operation change `out_vec`?
/* FP:bit_set.rs-1176 */ #[inline]
/* FP:bit_set.rs-1177 */ fn bitwise_changes<Op>(out_vec: &[Word], in_vec: &[Word], op: Op) -> bool
/* FP:bit_set.rs-1178 */ where
/* FP:bit_set.rs-1179 */     Op: Fn(Word, Word) -> Word,
/* FP:bit_set.rs-1180 */ {
/* FP:bit_set.rs-1181 */     assert_eq!(out_vec.len(), in_vec.len());
/* FP:bit_set.rs-1182 */     for (out_elem, in_elem) in iter::zip(out_vec, in_vec) {
/* FP:bit_set.rs-1183 */         let old_val = *out_elem;
/* FP:bit_set.rs-1184 */         let new_val = op(old_val, *in_elem);
/* FP:bit_set.rs-1185 */         if old_val != new_val {
/* FP:bit_set.rs-1186 */             return true;
/* FP:bit_set.rs-1187 */         }
/* FP:bit_set.rs-1188 */     }
/* FP:bit_set.rs-1189 */     false
/* FP:bit_set.rs-1190 */ }
/* FP:bit_set.rs-1191 */ 
/* FP:bit_set.rs-1192 */ /// A bitset with a mixed representation, using `DenseBitSet` for small and
/* FP:bit_set.rs-1193 */ /// medium bitsets, and `ChunkedBitSet` for large bitsets, i.e. those with
/* FP:bit_set.rs-1194 */ /// enough bits for at least two chunks. This is a good choice for many bitsets
/* FP:bit_set.rs-1195 */ /// that can have large domain sizes (e.g. 5000+).
/* FP:bit_set.rs-1196 */ ///
/* FP:bit_set.rs-1197 */ /// `T` is an index type, typically a newtyped `usize` wrapper, but it can also
/* FP:bit_set.rs-1198 */ /// just be `usize`.
/* FP:bit_set.rs-1199 */ ///
/* FP:bit_set.rs-1200 */ /// All operations that involve an element will panic if the element is equal
/* FP:bit_set.rs-1201 */ /// to or greater than the domain size. All operations that involve two bitsets
/* FP:bit_set.rs-1202 */ /// will panic if the bitsets have differing domain sizes.
/* FP:bit_set.rs-1203 */ #[derive(PartialEq, Eq)]
/* FP:bit_set.rs-1204 */ pub enum MixedBitSet<T> {
/* FP:bit_set.rs-1205 */     Small(DenseBitSet<T>),
/* FP:bit_set.rs-1206 */     Large(ChunkedBitSet<T>),
/* FP:bit_set.rs-1207 */ }
/* FP:bit_set.rs-1208 */ 
/* FP:bit_set.rs-1209 */ impl<T> MixedBitSet<T> {
/* FP:bit_set.rs-1210 */     pub fn domain_size(&self) -> usize {
/* FP:bit_set.rs-1211 */         match self {
/* FP:bit_set.rs-1212 */             MixedBitSet::Small(set) => set.domain_size(),
/* FP:bit_set.rs-1213 */             MixedBitSet::Large(set) => set.domain_size(),
/* FP:bit_set.rs-1214 */         }
/* FP:bit_set.rs-1215 */     }
/* FP:bit_set.rs-1216 */ }
/* FP:bit_set.rs-1217 */ 
/* FP:bit_set.rs-1218 */ impl<T: Idx> MixedBitSet<T> {
/* FP:bit_set.rs-1219 */     #[inline]
/* FP:bit_set.rs-1220 */     pub fn new_empty(domain_size: usize) -> MixedBitSet<T> {
/* FP:bit_set.rs-1221 */         if domain_size <= CHUNK_BITS {
/* FP:bit_set.rs-1222 */             MixedBitSet::Small(DenseBitSet::new_empty(domain_size))
/* FP:bit_set.rs-1223 */         } else {
/* FP:bit_set.rs-1224 */             MixedBitSet::Large(ChunkedBitSet::new_empty(domain_size))
/* FP:bit_set.rs-1225 */         }
/* FP:bit_set.rs-1226 */     }
/* FP:bit_set.rs-1227 */ 
/* FP:bit_set.rs-1228 */     #[inline]
/* FP:bit_set.rs-1229 */     pub fn is_empty(&self) -> bool {
/* FP:bit_set.rs-1230 */         match self {
/* FP:bit_set.rs-1231 */             MixedBitSet::Small(set) => set.is_empty(),
/* FP:bit_set.rs-1232 */             MixedBitSet::Large(set) => set.is_empty(),
/* FP:bit_set.rs-1233 */         }
/* FP:bit_set.rs-1234 */     }
/* FP:bit_set.rs-1235 */ 
/* FP:bit_set.rs-1236 */     #[inline]
/* FP:bit_set.rs-1237 */     pub fn contains(&self, elem: T) -> bool {
/* FP:bit_set.rs-1238 */         match self {
/* FP:bit_set.rs-1239 */             MixedBitSet::Small(set) => set.contains(elem),
/* FP:bit_set.rs-1240 */             MixedBitSet::Large(set) => set.contains(elem),
/* FP:bit_set.rs-1241 */         }
/* FP:bit_set.rs-1242 */     }
/* FP:bit_set.rs-1243 */ 
/* FP:bit_set.rs-1244 */     #[inline]
/* FP:bit_set.rs-1245 */     pub fn insert(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-1246 */         match self {
/* FP:bit_set.rs-1247 */             MixedBitSet::Small(set) => set.insert(elem),
/* FP:bit_set.rs-1248 */             MixedBitSet::Large(set) => set.insert(elem),
/* FP:bit_set.rs-1249 */         }
/* FP:bit_set.rs-1250 */     }
/* FP:bit_set.rs-1251 */ 
/* FP:bit_set.rs-1252 */     pub fn insert_all(&mut self) {
/* FP:bit_set.rs-1253 */         match self {
/* FP:bit_set.rs-1254 */             MixedBitSet::Small(set) => set.insert_all(),
/* FP:bit_set.rs-1255 */             MixedBitSet::Large(set) => set.insert_all(),
/* FP:bit_set.rs-1256 */         }
/* FP:bit_set.rs-1257 */     }
/* FP:bit_set.rs-1258 */ 
/* FP:bit_set.rs-1259 */     #[inline]
/* FP:bit_set.rs-1260 */     pub fn remove(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-1261 */         match self {
/* FP:bit_set.rs-1262 */             MixedBitSet::Small(set) => set.remove(elem),
/* FP:bit_set.rs-1263 */             MixedBitSet::Large(set) => set.remove(elem),
/* FP:bit_set.rs-1264 */         }
/* FP:bit_set.rs-1265 */     }
/* FP:bit_set.rs-1266 */ 
/* FP:bit_set.rs-1267 */     pub fn iter(&self) -> MixedBitIter<'_, T> {
/* FP:bit_set.rs-1268 */         match self {
/* FP:bit_set.rs-1269 */             MixedBitSet::Small(set) => MixedBitIter::Small(set.iter()),
/* FP:bit_set.rs-1270 */             MixedBitSet::Large(set) => MixedBitIter::Large(set.iter()),
/* FP:bit_set.rs-1271 */         }
/* FP:bit_set.rs-1272 */     }
/* FP:bit_set.rs-1273 */ 
/* FP:bit_set.rs-1274 */     #[inline]
/* FP:bit_set.rs-1275 */     pub fn clear(&mut self) {
/* FP:bit_set.rs-1276 */         match self {
/* FP:bit_set.rs-1277 */             MixedBitSet::Small(set) => set.clear(),
/* FP:bit_set.rs-1278 */             MixedBitSet::Large(set) => set.clear(),
/* FP:bit_set.rs-1279 */         }
/* FP:bit_set.rs-1280 */     }
/* FP:bit_set.rs-1281 */ 
/* FP:bit_set.rs-1282 */     bit_relations_inherent_impls! {}
/* FP:bit_set.rs-1283 */ }
/* FP:bit_set.rs-1284 */ 
/* FP:bit_set.rs-1285 */ impl<T> Clone for MixedBitSet<T> {
/* FP:bit_set.rs-1286 */     fn clone(&self) -> Self {
/* FP:bit_set.rs-1287 */         match self {
/* FP:bit_set.rs-1288 */             MixedBitSet::Small(set) => MixedBitSet::Small(set.clone()),
/* FP:bit_set.rs-1289 */             MixedBitSet::Large(set) => MixedBitSet::Large(set.clone()),
/* FP:bit_set.rs-1290 */         }
/* FP:bit_set.rs-1291 */     }
/* FP:bit_set.rs-1292 */ 
/* FP:bit_set.rs-1293 */     /// WARNING: this implementation of clone_from may panic if the two
/* FP:bit_set.rs-1294 */     /// bitsets have different domain sizes. This constraint is not inherent to
/* FP:bit_set.rs-1295 */     /// `clone_from`, but it works with the existing call sites and allows a
/* FP:bit_set.rs-1296 */     /// faster implementation, which is important because this function is hot.
/* FP:bit_set.rs-1297 */     fn clone_from(&mut self, from: &Self) {
/* FP:bit_set.rs-1298 */         match (self, from) {
/* FP:bit_set.rs-1299 */             (MixedBitSet::Small(set), MixedBitSet::Small(from)) => set.clone_from(from),
/* FP:bit_set.rs-1300 */             (MixedBitSet::Large(set), MixedBitSet::Large(from)) => set.clone_from(from),
/* FP:bit_set.rs-1301 */             _ => panic!("MixedBitSet size mismatch"),
/* FP:bit_set.rs-1302 */         }
/* FP:bit_set.rs-1303 */     }
/* FP:bit_set.rs-1304 */ }
/* FP:bit_set.rs-1305 */ 
/* FP:bit_set.rs-1306 */ impl<T: Idx> BitRelations<MixedBitSet<T>> for MixedBitSet<T> {
/* FP:bit_set.rs-1307 */     fn union(&mut self, other: &MixedBitSet<T>) -> bool {
/* FP:bit_set.rs-1308 */         match (self, other) {
/* FP:bit_set.rs-1309 */             (MixedBitSet::Small(set), MixedBitSet::Small(other)) => set.union(other),
/* FP:bit_set.rs-1310 */             (MixedBitSet::Large(set), MixedBitSet::Large(other)) => set.union(other),
/* FP:bit_set.rs-1311 */             _ => panic!("MixedBitSet size mismatch"),
/* FP:bit_set.rs-1312 */         }
/* FP:bit_set.rs-1313 */     }
/* FP:bit_set.rs-1314 */ 
/* FP:bit_set.rs-1315 */     fn subtract(&mut self, other: &MixedBitSet<T>) -> bool {
/* FP:bit_set.rs-1316 */         match (self, other) {
/* FP:bit_set.rs-1317 */             (MixedBitSet::Small(set), MixedBitSet::Small(other)) => set.subtract(other),
/* FP:bit_set.rs-1318 */             (MixedBitSet::Large(set), MixedBitSet::Large(other)) => set.subtract(other),
/* FP:bit_set.rs-1319 */             _ => panic!("MixedBitSet size mismatch"),
/* FP:bit_set.rs-1320 */         }
/* FP:bit_set.rs-1321 */     }
/* FP:bit_set.rs-1322 */ 
/* FP:bit_set.rs-1323 */     fn intersect(&mut self, _other: &MixedBitSet<T>) -> bool {
/* FP:bit_set.rs-1324 */         unimplemented!("implement if/when necessary");
/* FP:bit_set.rs-1325 */     }
/* FP:bit_set.rs-1326 */ }
/* FP:bit_set.rs-1327 */ 
/* FP:bit_set.rs-1328 */ impl<T: Idx> fmt::Debug for MixedBitSet<T> {
/* FP:bit_set.rs-1329 */     fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-1330 */         match self {
/* FP:bit_set.rs-1331 */             MixedBitSet::Small(set) => set.fmt(w),
/* FP:bit_set.rs-1332 */             MixedBitSet::Large(set) => set.fmt(w),
/* FP:bit_set.rs-1333 */         }
/* FP:bit_set.rs-1334 */     }
/* FP:bit_set.rs-1335 */ }
/* FP:bit_set.rs-1336 */ 
/* FP:bit_set.rs-1337 */ pub enum MixedBitIter<'a, T: Idx> {
/* FP:bit_set.rs-1338 */     Small(BitIter<'a, T>),
/* FP:bit_set.rs-1339 */     Large(ChunkedBitIter<'a, T>),
/* FP:bit_set.rs-1340 */ }
/* FP:bit_set.rs-1341 */ 
/* FP:bit_set.rs-1342 */ impl<'a, T: Idx> Iterator for MixedBitIter<'a, T> {
/* FP:bit_set.rs-1343 */     type Item = T;
/* FP:bit_set.rs-1344 */     fn next(&mut self) -> Option<T> {
/* FP:bit_set.rs-1345 */         match self {
/* FP:bit_set.rs-1346 */             MixedBitIter::Small(iter) => iter.next(),
/* FP:bit_set.rs-1347 */             MixedBitIter::Large(iter) => iter.next(),
/* FP:bit_set.rs-1348 */         }
/* FP:bit_set.rs-1349 */     }
/* FP:bit_set.rs-1350 */ }
/* FP:bit_set.rs-1351 */ 
/* FP:bit_set.rs-1352 */ /// A resizable bitset type with a dense representation.
/* FP:bit_set.rs-1353 */ ///
/* FP:bit_set.rs-1354 */ /// `T` is an index type, typically a newtyped `usize` wrapper, but it can also
/* FP:bit_set.rs-1355 */ /// just be `usize`.
/* FP:bit_set.rs-1356 */ ///
/* FP:bit_set.rs-1357 */ /// All operations that involve an element will panic if the element is equal
/* FP:bit_set.rs-1358 */ /// to or greater than the domain size.
/* FP:bit_set.rs-1359 */ #[derive(Clone, Debug, PartialEq)]
/* FP:bit_set.rs-1360 */ pub struct GrowableBitSet<T: Idx> {
/* FP:bit_set.rs-1361 */     bit_set: DenseBitSet<T>,
/* FP:bit_set.rs-1362 */ }
/* FP:bit_set.rs-1363 */ 
/* FP:bit_set.rs-1364 */ impl<T: Idx> Default for GrowableBitSet<T> {
/* FP:bit_set.rs-1365 */     fn default() -> Self {
/* FP:bit_set.rs-1366 */         GrowableBitSet::new_empty()
/* FP:bit_set.rs-1367 */     }
/* FP:bit_set.rs-1368 */ }
/* FP:bit_set.rs-1369 */ 
/* FP:bit_set.rs-1370 */ impl<T: Idx> GrowableBitSet<T> {
/* FP:bit_set.rs-1371 */     /// Ensure that the set can hold at least `min_domain_size` elements.
/* FP:bit_set.rs-1372 */     pub fn ensure(&mut self, min_domain_size: usize) {
/* FP:bit_set.rs-1373 */         if self.bit_set.domain_size < min_domain_size {
/* FP:bit_set.rs-1374 */             self.bit_set.domain_size = min_domain_size;
/* FP:bit_set.rs-1375 */         }
/* FP:bit_set.rs-1376 */ 
/* FP:bit_set.rs-1377 */         let min_num_words = num_words(min_domain_size);
/* FP:bit_set.rs-1378 */         if self.bit_set.words.len() < min_num_words {
/* FP:bit_set.rs-1379 */             self.bit_set.words.resize(min_num_words, 0)
/* FP:bit_set.rs-1380 */         }
/* FP:bit_set.rs-1381 */     }
/* FP:bit_set.rs-1382 */ 
/* FP:bit_set.rs-1383 */     pub fn new_empty() -> GrowableBitSet<T> {
/* FP:bit_set.rs-1384 */         GrowableBitSet { bit_set: DenseBitSet::new_empty(0) }
/* FP:bit_set.rs-1385 */     }
/* FP:bit_set.rs-1386 */ 
/* FP:bit_set.rs-1387 */     pub fn with_capacity(capacity: usize) -> GrowableBitSet<T> {
/* FP:bit_set.rs-1388 */         GrowableBitSet { bit_set: DenseBitSet::new_empty(capacity) }
/* FP:bit_set.rs-1389 */     }
/* FP:bit_set.rs-1390 */ 
/* FP:bit_set.rs-1391 */     /// Returns `true` if the set has changed.
/* FP:bit_set.rs-1392 */     #[inline]
/* FP:bit_set.rs-1393 */     pub fn insert(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-1394 */         self.ensure(elem.index() + 1);
/* FP:bit_set.rs-1395 */         self.bit_set.insert(elem)
/* FP:bit_set.rs-1396 */     }
/* FP:bit_set.rs-1397 */ 
/* FP:bit_set.rs-1398 */     /// Returns `true` if the set has changed.
/* FP:bit_set.rs-1399 */     #[inline]
/* FP:bit_set.rs-1400 */     pub fn remove(&mut self, elem: T) -> bool {
/* FP:bit_set.rs-1401 */         self.ensure(elem.index() + 1);
/* FP:bit_set.rs-1402 */         self.bit_set.remove(elem)
/* FP:bit_set.rs-1403 */     }
/* FP:bit_set.rs-1404 */ 
/* FP:bit_set.rs-1405 */     #[inline]
/* FP:bit_set.rs-1406 */     pub fn is_empty(&self) -> bool {
/* FP:bit_set.rs-1407 */         self.bit_set.is_empty()
/* FP:bit_set.rs-1408 */     }
/* FP:bit_set.rs-1409 */ 
/* FP:bit_set.rs-1410 */     #[inline]
/* FP:bit_set.rs-1411 */     pub fn contains(&self, elem: T) -> bool {
/* FP:bit_set.rs-1412 */         let (word_index, mask) = word_index_and_mask(elem);
/* FP:bit_set.rs-1413 */         self.bit_set.words.get(word_index).is_some_and(|word| (word & mask) != 0)
/* FP:bit_set.rs-1414 */     }
/* FP:bit_set.rs-1415 */ 
/* FP:bit_set.rs-1416 */     #[inline]
/* FP:bit_set.rs-1417 */     pub fn iter(&self) -> BitIter<'_, T> {
/* FP:bit_set.rs-1418 */         self.bit_set.iter()
/* FP:bit_set.rs-1419 */     }
/* FP:bit_set.rs-1420 */ 
/* FP:bit_set.rs-1421 */     #[inline]
/* FP:bit_set.rs-1422 */     pub fn len(&self) -> usize {
/* FP:bit_set.rs-1423 */         self.bit_set.count()
/* FP:bit_set.rs-1424 */     }
/* FP:bit_set.rs-1425 */ }
/* FP:bit_set.rs-1426 */ 
/* FP:bit_set.rs-1427 */ impl<T: Idx> From<DenseBitSet<T>> for GrowableBitSet<T> {
/* FP:bit_set.rs-1428 */     fn from(bit_set: DenseBitSet<T>) -> Self {
/* FP:bit_set.rs-1429 */         Self { bit_set }
/* FP:bit_set.rs-1430 */     }
/* FP:bit_set.rs-1431 */ }
/* FP:bit_set.rs-1432 */ 
/* FP:bit_set.rs-1433 */ /// A fixed-size 2D bit matrix type with a dense representation.
/* FP:bit_set.rs-1434 */ ///
/* FP:bit_set.rs-1435 */ /// `R` and `C` are index types used to identify rows and columns respectively;
/* FP:bit_set.rs-1436 */ /// typically newtyped `usize` wrappers, but they can also just be `usize`.
/* FP:bit_set.rs-1437 */ ///
/* FP:bit_set.rs-1438 */ /// All operations that involve a row and/or column index will panic if the
/* FP:bit_set.rs-1439 */ /// index exceeds the relevant bound.
/* FP:bit_set.rs-1440 */ #[cfg_attr(feature = "nightly", derive(Decodable_NoContext, Encodable_NoContext))]
/* FP:bit_set.rs-1441 */ #[derive(Clone, Eq, PartialEq, Hash)]
/* FP:bit_set.rs-1442 */ pub struct BitMatrix<R: Idx, C: Idx> {
/* FP:bit_set.rs-1443 */     num_rows: usize,
/* FP:bit_set.rs-1444 */     num_columns: usize,
/* FP:bit_set.rs-1445 */     words: SmallVec<[Word; 2]>,
/* FP:bit_set.rs-1446 */     marker: PhantomData<(R, C)>,
/* FP:bit_set.rs-1447 */ }
/* FP:bit_set.rs-1448 */ 
/* FP:bit_set.rs-1449 */ impl<R: Idx, C: Idx> BitMatrix<R, C> {
/* FP:bit_set.rs-1450 */     /// Creates a new `rows x columns` matrix, initially empty.
/* FP:bit_set.rs-1451 */     pub fn new(num_rows: usize, num_columns: usize) -> BitMatrix<R, C> {
/* FP:bit_set.rs-1452 */         // For every element, we need one bit for every other
/* FP:bit_set.rs-1453 */         // element. Round up to an even number of words.
/* FP:bit_set.rs-1454 */         let words_per_row = num_words(num_columns);
/* FP:bit_set.rs-1455 */         BitMatrix {
/* FP:bit_set.rs-1456 */             num_rows,
/* FP:bit_set.rs-1457 */             num_columns,
/* FP:bit_set.rs-1458 */             words: smallvec![0; num_rows * words_per_row],
/* FP:bit_set.rs-1459 */             marker: PhantomData,
/* FP:bit_set.rs-1460 */         }
/* FP:bit_set.rs-1461 */     }
/* FP:bit_set.rs-1462 */ 
/* FP:bit_set.rs-1463 */     /// Creates a new matrix, with `row` used as the value for every row.
/* FP:bit_set.rs-1464 */     pub fn from_row_n(row: &DenseBitSet<C>, num_rows: usize) -> BitMatrix<R, C> {
/* FP:bit_set.rs-1465 */         let num_columns = row.domain_size();
/* FP:bit_set.rs-1466 */         let words_per_row = num_words(num_columns);
/* FP:bit_set.rs-1467 */         assert_eq!(words_per_row, row.words.len());
/* FP:bit_set.rs-1468 */         BitMatrix {
/* FP:bit_set.rs-1469 */             num_rows,
/* FP:bit_set.rs-1470 */             num_columns,
/* FP:bit_set.rs-1471 */             words: iter::repeat(&row.words).take(num_rows).flatten().cloned().collect(),
/* FP:bit_set.rs-1472 */             marker: PhantomData,
/* FP:bit_set.rs-1473 */         }
/* FP:bit_set.rs-1474 */     }
/* FP:bit_set.rs-1475 */ 
/* FP:bit_set.rs-1476 */     pub fn rows(&self) -> impl Iterator<Item = R> {
/* FP:bit_set.rs-1477 */         (0..self.num_rows).map(R::new)
/* FP:bit_set.rs-1478 */     }
/* FP:bit_set.rs-1479 */ 
/* FP:bit_set.rs-1480 */     /// The range of bits for a given row.
/* FP:bit_set.rs-1481 */     fn range(&self, row: R) -> (usize, usize) {
/* FP:bit_set.rs-1482 */         let words_per_row = num_words(self.num_columns);
/* FP:bit_set.rs-1483 */         let start = row.index() * words_per_row;
/* FP:bit_set.rs-1484 */         (start, start + words_per_row)
/* FP:bit_set.rs-1485 */     }
/* FP:bit_set.rs-1486 */ 
/* FP:bit_set.rs-1487 */     /// Sets the cell at `(row, column)` to true. Put another way, insert
/* FP:bit_set.rs-1488 */     /// `column` to the bitset for `row`.
/* FP:bit_set.rs-1489 */     ///
/* FP:bit_set.rs-1490 */     /// Returns `true` if this changed the matrix.
/* FP:bit_set.rs-1491 */     pub fn insert(&mut self, row: R, column: C) -> bool {
/* FP:bit_set.rs-1492 */         assert!(row.index() < self.num_rows && column.index() < self.num_columns);
/* FP:bit_set.rs-1493 */         let (start, _) = self.range(row);
/* FP:bit_set.rs-1494 */         let (word_index, mask) = word_index_and_mask(column);
/* FP:bit_set.rs-1495 */         let words = &mut self.words[..];
/* FP:bit_set.rs-1496 */         let word = words[start + word_index];
/* FP:bit_set.rs-1497 */         let new_word = word | mask;
/* FP:bit_set.rs-1498 */         words[start + word_index] = new_word;
/* FP:bit_set.rs-1499 */         word != new_word
/* FP:bit_set.rs-1500 */     }
/* FP:bit_set.rs-1501 */ 
/* FP:bit_set.rs-1502 */     /// Do the bits from `row` contain `column`? Put another way, is
/* FP:bit_set.rs-1503 */     /// the matrix cell at `(row, column)` true?  Put yet another way,
/* FP:bit_set.rs-1504 */     /// if the matrix represents (transitive) reachability, can
/* FP:bit_set.rs-1505 */     /// `row` reach `column`?
/* FP:bit_set.rs-1506 */     pub fn contains(&self, row: R, column: C) -> bool {
/* FP:bit_set.rs-1507 */         assert!(row.index() < self.num_rows && column.index() < self.num_columns);
/* FP:bit_set.rs-1508 */         let (start, _) = self.range(row);
/* FP:bit_set.rs-1509 */         let (word_index, mask) = word_index_and_mask(column);
/* FP:bit_set.rs-1510 */         (self.words[start + word_index] & mask) != 0
/* FP:bit_set.rs-1511 */     }
/* FP:bit_set.rs-1512 */ 
/* FP:bit_set.rs-1513 */     /// Returns those indices that are true in rows `a` and `b`. This
/* FP:bit_set.rs-1514 */     /// is an *O*(*n*) operation where *n* is the number of elements
/* FP:bit_set.rs-1515 */     /// (somewhat independent from the actual size of the
/* FP:bit_set.rs-1516 */     /// intersection, in particular).
/* FP:bit_set.rs-1517 */     pub fn intersect_rows(&self, row1: R, row2: R) -> Vec<C> {
/* FP:bit_set.rs-1518 */         assert!(row1.index() < self.num_rows && row2.index() < self.num_rows);
/* FP:bit_set.rs-1519 */         let (row1_start, row1_end) = self.range(row1);
/* FP:bit_set.rs-1520 */         let (row2_start, row2_end) = self.range(row2);
/* FP:bit_set.rs-1521 */         let mut result = Vec::with_capacity(self.num_columns);
/* FP:bit_set.rs-1522 */         for (base, (i, j)) in (row1_start..row1_end).zip(row2_start..row2_end).enumerate() {
/* FP:bit_set.rs-1523 */             let mut v = self.words[i] & self.words[j];
/* FP:bit_set.rs-1524 */             for bit in 0..WORD_BITS {
/* FP:bit_set.rs-1525 */                 if v == 0 {
/* FP:bit_set.rs-1526 */                     break;
/* FP:bit_set.rs-1527 */                 }
/* FP:bit_set.rs-1528 */                 if v & 0x1 != 0 {
/* FP:bit_set.rs-1529 */                     result.push(C::new(base * WORD_BITS + bit));
/* FP:bit_set.rs-1530 */                 }
/* FP:bit_set.rs-1531 */                 v >>= 1;
/* FP:bit_set.rs-1532 */             }
/* FP:bit_set.rs-1533 */         }
/* FP:bit_set.rs-1534 */         result
/* FP:bit_set.rs-1535 */     }
/* FP:bit_set.rs-1536 */ 
/* FP:bit_set.rs-1537 */     /// Adds the bits from row `read` to the bits from row `write`, and
/* FP:bit_set.rs-1538 */     /// returns `true` if anything changed.
/* FP:bit_set.rs-1539 */     ///
/* FP:bit_set.rs-1540 */     /// This is used when computing transitive reachability because if
/* FP:bit_set.rs-1541 */     /// you have an edge `write -> read`, because in that case
/* FP:bit_set.rs-1542 */     /// `write` can reach everything that `read` can (and
/* FP:bit_set.rs-1543 */     /// potentially more).
/* FP:bit_set.rs-1544 */     pub fn union_rows(&mut self, read: R, write: R) -> bool {
/* FP:bit_set.rs-1545 */         assert!(read.index() < self.num_rows && write.index() < self.num_rows);
/* FP:bit_set.rs-1546 */         let (read_start, read_end) = self.range(read);
/* FP:bit_set.rs-1547 */         let (write_start, write_end) = self.range(write);
/* FP:bit_set.rs-1548 */         let words = &mut self.words[..];
/* FP:bit_set.rs-1549 */         let mut changed = 0;
/* FP:bit_set.rs-1550 */         for (read_index, write_index) in iter::zip(read_start..read_end, write_start..write_end) {
/* FP:bit_set.rs-1551 */             let word = words[write_index];
/* FP:bit_set.rs-1552 */             let new_word = word | words[read_index];
/* FP:bit_set.rs-1553 */             words[write_index] = new_word;
/* FP:bit_set.rs-1554 */             // See `bitwise` for the rationale.
/* FP:bit_set.rs-1555 */             changed |= word ^ new_word;
/* FP:bit_set.rs-1556 */         }
/* FP:bit_set.rs-1557 */         changed != 0
/* FP:bit_set.rs-1558 */     }
/* FP:bit_set.rs-1559 */ 
/* FP:bit_set.rs-1560 */     /// Adds the bits from `with` to the bits from row `write`, and
/* FP:bit_set.rs-1561 */     /// returns `true` if anything changed.
/* FP:bit_set.rs-1562 */     pub fn union_row_with(&mut self, with: &DenseBitSet<C>, write: R) -> bool {
/* FP:bit_set.rs-1563 */         assert!(write.index() < self.num_rows);
/* FP:bit_set.rs-1564 */         assert_eq!(with.domain_size(), self.num_columns);
/* FP:bit_set.rs-1565 */         let (write_start, write_end) = self.range(write);
/* FP:bit_set.rs-1566 */         bitwise(&mut self.words[write_start..write_end], &with.words, |a, b| a | b)
/* FP:bit_set.rs-1567 */     }
/* FP:bit_set.rs-1568 */ 
/* FP:bit_set.rs-1569 */     /// Sets every cell in `row` to true.
/* FP:bit_set.rs-1570 */     pub fn insert_all_into_row(&mut self, row: R) {
/* FP:bit_set.rs-1571 */         assert!(row.index() < self.num_rows);
/* FP:bit_set.rs-1572 */         let (start, end) = self.range(row);
/* FP:bit_set.rs-1573 */         let words = &mut self.words[..];
/* FP:bit_set.rs-1574 */         for index in start..end {
/* FP:bit_set.rs-1575 */             words[index] = !0;
/* FP:bit_set.rs-1576 */         }
/* FP:bit_set.rs-1577 */         clear_excess_bits_in_final_word(self.num_columns, &mut self.words[..end]);
/* FP:bit_set.rs-1578 */     }
/* FP:bit_set.rs-1579 */ 
/* FP:bit_set.rs-1580 */     /// Gets a slice of the underlying words.
/* FP:bit_set.rs-1581 */     pub fn words(&self) -> &[Word] {
/* FP:bit_set.rs-1582 */         &self.words
/* FP:bit_set.rs-1583 */     }
/* FP:bit_set.rs-1584 */ 
/* FP:bit_set.rs-1585 */     /// Iterates through all the columns set to true in a given row of
/* FP:bit_set.rs-1586 */     /// the matrix.
/* FP:bit_set.rs-1587 */     pub fn iter(&self, row: R) -> BitIter<'_, C> {
/* FP:bit_set.rs-1588 */         assert!(row.index() < self.num_rows);
/* FP:bit_set.rs-1589 */         let (start, end) = self.range(row);
/* FP:bit_set.rs-1590 */         BitIter::new(&self.words[start..end])
/* FP:bit_set.rs-1591 */     }
/* FP:bit_set.rs-1592 */ 
/* FP:bit_set.rs-1593 */     /// Returns the number of elements in `row`.
/* FP:bit_set.rs-1594 */     pub fn count(&self, row: R) -> usize {
/* FP:bit_set.rs-1595 */         let (start, end) = self.range(row);
/* FP:bit_set.rs-1596 */         self.words[start..end].iter().map(|e| e.count_ones() as usize).sum()
/* FP:bit_set.rs-1597 */     }
/* FP:bit_set.rs-1598 */ }
/* FP:bit_set.rs-1599 */ 
/* FP:bit_set.rs-1600 */ impl<R: Idx, C: Idx> fmt::Debug for BitMatrix<R, C> {
/* FP:bit_set.rs-1601 */     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-1602 */         /// Forces its contents to print in regular mode instead of alternate mode.
/* FP:bit_set.rs-1603 */         struct OneLinePrinter<T>(T);
/* FP:bit_set.rs-1604 */         impl<T: fmt::Debug> fmt::Debug for OneLinePrinter<T> {
/* FP:bit_set.rs-1605 */             fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-1606 */                 write!(fmt, "{:?}", self.0)
/* FP:bit_set.rs-1607 */             }
/* FP:bit_set.rs-1608 */         }
/* FP:bit_set.rs-1609 */ 
/* FP:bit_set.rs-1610 */         write!(fmt, "BitMatrix({}x{}) ", self.num_rows, self.num_columns)?;
/* FP:bit_set.rs-1611 */         let items = self.rows().flat_map(|r| self.iter(r).map(move |c| (r, c)));
/* FP:bit_set.rs-1612 */         fmt.debug_set().entries(items.map(OneLinePrinter)).finish()
/* FP:bit_set.rs-1613 */     }
/* FP:bit_set.rs-1614 */ }
/* FP:bit_set.rs-1615 */ 
/* FP:bit_set.rs-1616 */ /// A fixed-column-size, variable-row-size 2D bit matrix with a moderately
/* FP:bit_set.rs-1617 */ /// sparse representation.
/* FP:bit_set.rs-1618 */ ///
/* FP:bit_set.rs-1619 */ /// Initially, every row has no explicit representation. If any bit within a row
/* FP:bit_set.rs-1620 */ /// is set, the entire row is instantiated as `Some(<DenseBitSet>)`.
/* FP:bit_set.rs-1621 */ /// Furthermore, any previously uninstantiated rows prior to it will be
/* FP:bit_set.rs-1622 */ /// instantiated as `None`. Those prior rows may themselves become fully
/* FP:bit_set.rs-1623 */ /// instantiated later on if any of their bits are set.
/* FP:bit_set.rs-1624 */ ///
/* FP:bit_set.rs-1625 */ /// `R` and `C` are index types used to identify rows and columns respectively;
/* FP:bit_set.rs-1626 */ /// typically newtyped `usize` wrappers, but they can also just be `usize`.
/* FP:bit_set.rs-1627 */ #[derive(Clone, Debug)]
/* FP:bit_set.rs-1628 */ pub struct SparseBitMatrix<R, C>
/* FP:bit_set.rs-1629 */ where
/* FP:bit_set.rs-1630 */     R: Idx,
/* FP:bit_set.rs-1631 */     C: Idx,
/* FP:bit_set.rs-1632 */ {
/* FP:bit_set.rs-1633 */     num_columns: usize,
/* FP:bit_set.rs-1634 */     rows: IndexVec<R, Option<DenseBitSet<C>>>,
/* FP:bit_set.rs-1635 */ }
/* FP:bit_set.rs-1636 */ 
/* FP:bit_set.rs-1637 */ impl<R: Idx, C: Idx> SparseBitMatrix<R, C> {
/* FP:bit_set.rs-1638 */     /// Creates a new empty sparse bit matrix with no rows or columns.
/* FP:bit_set.rs-1639 */     pub fn new(num_columns: usize) -> Self {
/* FP:bit_set.rs-1640 */         Self { num_columns, rows: IndexVec::new() }
/* FP:bit_set.rs-1641 */     }
/* FP:bit_set.rs-1642 */ 
/* FP:bit_set.rs-1643 */     fn ensure_row(&mut self, row: R) -> &mut DenseBitSet<C> {
/* FP:bit_set.rs-1644 */         // Instantiate any missing rows up to and including row `row` with an empty `DenseBitSet`.
/* FP:bit_set.rs-1645 */         // Then replace row `row` with a full `DenseBitSet` if necessary.
/* FP:bit_set.rs-1646 */         self.rows.get_or_insert_with(row, || DenseBitSet::new_empty(self.num_columns))
/* FP:bit_set.rs-1647 */     }
/* FP:bit_set.rs-1648 */ 
/* FP:bit_set.rs-1649 */     /// Sets the cell at `(row, column)` to true. Put another way, insert
/* FP:bit_set.rs-1650 */     /// `column` to the bitset for `row`.
/* FP:bit_set.rs-1651 */     ///
/* FP:bit_set.rs-1652 */     /// Returns `true` if this changed the matrix.
/* FP:bit_set.rs-1653 */     pub fn insert(&mut self, row: R, column: C) -> bool {
/* FP:bit_set.rs-1654 */         self.ensure_row(row).insert(column)
/* FP:bit_set.rs-1655 */     }
/* FP:bit_set.rs-1656 */ 
/* FP:bit_set.rs-1657 */     /// Sets the cell at `(row, column)` to false. Put another way, delete
/* FP:bit_set.rs-1658 */     /// `column` from the bitset for `row`. Has no effect if `row` does not
/* FP:bit_set.rs-1659 */     /// exist.
/* FP:bit_set.rs-1660 */     ///
/* FP:bit_set.rs-1661 */     /// Returns `true` if this changed the matrix.
/* FP:bit_set.rs-1662 */     pub fn remove(&mut self, row: R, column: C) -> bool {
/* FP:bit_set.rs-1663 */         match self.rows.get_mut(row) {
/* FP:bit_set.rs-1664 */             Some(Some(row)) => row.remove(column),
/* FP:bit_set.rs-1665 */             _ => false,
/* FP:bit_set.rs-1666 */         }
/* FP:bit_set.rs-1667 */     }
/* FP:bit_set.rs-1668 */ 
/* FP:bit_set.rs-1669 */     /// Sets all columns at `row` to false. Has no effect if `row` does
/* FP:bit_set.rs-1670 */     /// not exist.
/* FP:bit_set.rs-1671 */     pub fn clear(&mut self, row: R) {
/* FP:bit_set.rs-1672 */         if let Some(Some(row)) = self.rows.get_mut(row) {
/* FP:bit_set.rs-1673 */             row.clear();
/* FP:bit_set.rs-1674 */         }
/* FP:bit_set.rs-1675 */     }
/* FP:bit_set.rs-1676 */ 
/* FP:bit_set.rs-1677 */     /// Do the bits from `row` contain `column`? Put another way, is
/* FP:bit_set.rs-1678 */     /// the matrix cell at `(row, column)` true?  Put yet another way,
/* FP:bit_set.rs-1679 */     /// if the matrix represents (transitive) reachability, can
/* FP:bit_set.rs-1680 */     /// `row` reach `column`?
/* FP:bit_set.rs-1681 */     pub fn contains(&self, row: R, column: C) -> bool {
/* FP:bit_set.rs-1682 */         self.row(row).is_some_and(|r| r.contains(column))
/* FP:bit_set.rs-1683 */     }
/* FP:bit_set.rs-1684 */ 
/* FP:bit_set.rs-1685 */     /// Adds the bits from row `read` to the bits from row `write`, and
/* FP:bit_set.rs-1686 */     /// returns `true` if anything changed.
/* FP:bit_set.rs-1687 */     ///
/* FP:bit_set.rs-1688 */     /// This is used when computing transitive reachability because if
/* FP:bit_set.rs-1689 */     /// you have an edge `write -> read`, because in that case
/* FP:bit_set.rs-1690 */     /// `write` can reach everything that `read` can (and
/* FP:bit_set.rs-1691 */     /// potentially more).
/* FP:bit_set.rs-1692 */     pub fn union_rows(&mut self, read: R, write: R) -> bool {
/* FP:bit_set.rs-1693 */         if read == write || self.row(read).is_none() {
/* FP:bit_set.rs-1694 */             return false;
/* FP:bit_set.rs-1695 */         }
/* FP:bit_set.rs-1696 */ 
/* FP:bit_set.rs-1697 */         self.ensure_row(write);
/* FP:bit_set.rs-1698 */         if let (Some(read_row), Some(write_row)) = self.rows.pick2_mut(read, write) {
/* FP:bit_set.rs-1699 */             write_row.union(read_row)
/* FP:bit_set.rs-1700 */         } else {
/* FP:bit_set.rs-1701 */             unreachable!()
/* FP:bit_set.rs-1702 */         }
/* FP:bit_set.rs-1703 */     }
/* FP:bit_set.rs-1704 */ 
/* FP:bit_set.rs-1705 */     /// Insert all bits in the given row.
/* FP:bit_set.rs-1706 */     pub fn insert_all_into_row(&mut self, row: R) {
/* FP:bit_set.rs-1707 */         self.ensure_row(row).insert_all();
/* FP:bit_set.rs-1708 */     }
/* FP:bit_set.rs-1709 */ 
/* FP:bit_set.rs-1710 */     pub fn rows(&self) -> impl Iterator<Item = R> {
/* FP:bit_set.rs-1711 */         self.rows.indices()
/* FP:bit_set.rs-1712 */     }
/* FP:bit_set.rs-1713 */ 
/* FP:bit_set.rs-1714 */     /// Iterates through all the columns set to true in a given row of
/* FP:bit_set.rs-1715 */     /// the matrix.
/* FP:bit_set.rs-1716 */     pub fn iter(&self, row: R) -> impl Iterator<Item = C> {
/* FP:bit_set.rs-1717 */         self.row(row).into_iter().flat_map(|r| r.iter())
/* FP:bit_set.rs-1718 */     }
/* FP:bit_set.rs-1719 */ 
/* FP:bit_set.rs-1720 */     pub fn row(&self, row: R) -> Option<&DenseBitSet<C>> {
/* FP:bit_set.rs-1721 */         self.rows.get(row)?.as_ref()
/* FP:bit_set.rs-1722 */     }
/* FP:bit_set.rs-1723 */ 
/* FP:bit_set.rs-1724 */     /// Intersects `row` with `set`. `set` can be either `DenseBitSet` or
/* FP:bit_set.rs-1725 */     /// `ChunkedBitSet`. Has no effect if `row` does not exist.
/* FP:bit_set.rs-1726 */     ///
/* FP:bit_set.rs-1727 */     /// Returns true if the row was changed.
/* FP:bit_set.rs-1728 */     pub fn intersect_row<Set>(&mut self, row: R, set: &Set) -> bool
/* FP:bit_set.rs-1729 */     where
/* FP:bit_set.rs-1730 */         DenseBitSet<C>: BitRelations<Set>,
/* FP:bit_set.rs-1731 */     {
/* FP:bit_set.rs-1732 */         match self.rows.get_mut(row) {
/* FP:bit_set.rs-1733 */             Some(Some(row)) => row.intersect(set),
/* FP:bit_set.rs-1734 */             _ => false,
/* FP:bit_set.rs-1735 */         }
/* FP:bit_set.rs-1736 */     }
/* FP:bit_set.rs-1737 */ 
/* FP:bit_set.rs-1738 */     /// Subtracts `set` from `row`. `set` can be either `DenseBitSet` or
/* FP:bit_set.rs-1739 */     /// `ChunkedBitSet`. Has no effect if `row` does not exist.
/* FP:bit_set.rs-1740 */     ///
/* FP:bit_set.rs-1741 */     /// Returns true if the row was changed.
/* FP:bit_set.rs-1742 */     pub fn subtract_row<Set>(&mut self, row: R, set: &Set) -> bool
/* FP:bit_set.rs-1743 */     where
/* FP:bit_set.rs-1744 */         DenseBitSet<C>: BitRelations<Set>,
/* FP:bit_set.rs-1745 */     {
/* FP:bit_set.rs-1746 */         match self.rows.get_mut(row) {
/* FP:bit_set.rs-1747 */             Some(Some(row)) => row.subtract(set),
/* FP:bit_set.rs-1748 */             _ => false,
/* FP:bit_set.rs-1749 */         }
/* FP:bit_set.rs-1750 */     }
/* FP:bit_set.rs-1751 */ 
/* FP:bit_set.rs-1752 */     /// Unions `row` with `set`. `set` can be either `DenseBitSet` or
/* FP:bit_set.rs-1753 */     /// `ChunkedBitSet`.
/* FP:bit_set.rs-1754 */     ///
/* FP:bit_set.rs-1755 */     /// Returns true if the row was changed.
/* FP:bit_set.rs-1756 */     pub fn union_row<Set>(&mut self, row: R, set: &Set) -> bool
/* FP:bit_set.rs-1757 */     where
/* FP:bit_set.rs-1758 */         DenseBitSet<C>: BitRelations<Set>,
/* FP:bit_set.rs-1759 */     {
/* FP:bit_set.rs-1760 */         self.ensure_row(row).union(set)
/* FP:bit_set.rs-1761 */     }
/* FP:bit_set.rs-1762 */ }
/* FP:bit_set.rs-1763 */ 
/* FP:bit_set.rs-1764 */ #[inline]
/* FP:bit_set.rs-1765 */ fn num_words<T: Idx>(domain_size: T) -> usize {
/* FP:bit_set.rs-1766 */     domain_size.index().div_ceil(WORD_BITS)
/* FP:bit_set.rs-1767 */ }
/* FP:bit_set.rs-1768 */ 
/* FP:bit_set.rs-1769 */ #[inline]
/* FP:bit_set.rs-1770 */ fn num_chunks<T: Idx>(domain_size: T) -> usize {
/* FP:bit_set.rs-1771 */     assert!(domain_size.index() > 0);
/* FP:bit_set.rs-1772 */     domain_size.index().div_ceil(CHUNK_BITS)
/* FP:bit_set.rs-1773 */ }
/* FP:bit_set.rs-1774 */ 
/* FP:bit_set.rs-1775 */ #[inline]
/* FP:bit_set.rs-1776 */ fn word_index_and_mask<T: Idx>(elem: T) -> (usize, Word) {
/* FP:bit_set.rs-1777 */     let elem = elem.index();
/* FP:bit_set.rs-1778 */     let word_index = elem / WORD_BITS;
/* FP:bit_set.rs-1779 */     let mask = 1 << (elem % WORD_BITS);
/* FP:bit_set.rs-1780 */     (word_index, mask)
/* FP:bit_set.rs-1781 */ }
/* FP:bit_set.rs-1782 */ 
/* FP:bit_set.rs-1783 */ #[inline]
/* FP:bit_set.rs-1784 */ fn chunk_index<T: Idx>(elem: T) -> usize {
/* FP:bit_set.rs-1785 */     elem.index() / CHUNK_BITS
/* FP:bit_set.rs-1786 */ }
/* FP:bit_set.rs-1787 */ 
/* FP:bit_set.rs-1788 */ #[inline]
/* FP:bit_set.rs-1789 */ fn chunk_word_index_and_mask<T: Idx>(elem: T) -> (usize, Word) {
/* FP:bit_set.rs-1790 */     let chunk_elem = elem.index() % CHUNK_BITS;
/* FP:bit_set.rs-1791 */     word_index_and_mask(chunk_elem)
/* FP:bit_set.rs-1792 */ }
/* FP:bit_set.rs-1793 */ 
/* FP:bit_set.rs-1794 */ fn clear_excess_bits_in_final_word(domain_size: usize, words: &mut [Word]) {
/* FP:bit_set.rs-1795 */     let num_bits_in_final_word = domain_size % WORD_BITS;
/* FP:bit_set.rs-1796 */     if num_bits_in_final_word > 0 {
/* FP:bit_set.rs-1797 */         let mask = (1 << num_bits_in_final_word) - 1;
/* FP:bit_set.rs-1798 */         words[words.len() - 1] &= mask;
/* FP:bit_set.rs-1799 */     }
/* FP:bit_set.rs-1800 */ }
/* FP:bit_set.rs-1801 */ 
/* FP:bit_set.rs-1802 */ #[inline]
/* FP:bit_set.rs-1803 */ fn max_bit(word: Word) -> usize {
/* FP:bit_set.rs-1804 */     WORD_BITS - 1 - word.leading_zeros() as usize
/* FP:bit_set.rs-1805 */ }
/* FP:bit_set.rs-1806 */ 
/* FP:bit_set.rs-1807 */ /// Integral type used to represent the bit set.
/* FP:bit_set.rs-1808 */ pub trait FiniteBitSetTy:
/* FP:bit_set.rs-1809 */     BitAnd<Output = Self>
/* FP:bit_set.rs-1810 */     + BitAndAssign
/* FP:bit_set.rs-1811 */     + BitOrAssign
/* FP:bit_set.rs-1812 */     + Clone
/* FP:bit_set.rs-1813 */     + Copy
/* FP:bit_set.rs-1814 */     + Shl
/* FP:bit_set.rs-1815 */     + Not<Output = Self>
/* FP:bit_set.rs-1816 */     + PartialEq
/* FP:bit_set.rs-1817 */     + Sized
/* FP:bit_set.rs-1818 */ {
/* FP:bit_set.rs-1819 */     /// Size of the domain representable by this type, e.g. 64 for `u64`.
/* FP:bit_set.rs-1820 */     const DOMAIN_SIZE: u32;
/* FP:bit_set.rs-1821 */ 
/* FP:bit_set.rs-1822 */     /// Value which represents the `FiniteBitSet` having every bit set.
/* FP:bit_set.rs-1823 */     const FILLED: Self;
/* FP:bit_set.rs-1824 */     /// Value which represents the `FiniteBitSet` having no bits set.
/* FP:bit_set.rs-1825 */     const EMPTY: Self;
/* FP:bit_set.rs-1826 */ 
/* FP:bit_set.rs-1827 */     /// Value for one as the integral type.
/* FP:bit_set.rs-1828 */     const ONE: Self;
/* FP:bit_set.rs-1829 */     /// Value for zero as the integral type.
/* FP:bit_set.rs-1830 */     const ZERO: Self;
/* FP:bit_set.rs-1831 */ 
/* FP:bit_set.rs-1832 */     /// Perform a checked left shift on the integral type.
/* FP:bit_set.rs-1833 */     fn checked_shl(self, rhs: u32) -> Option<Self>;
/* FP:bit_set.rs-1834 */     /// Perform a checked right shift on the integral type.
/* FP:bit_set.rs-1835 */     fn checked_shr(self, rhs: u32) -> Option<Self>;
/* FP:bit_set.rs-1836 */ }
/* FP:bit_set.rs-1837 */ 
/* FP:bit_set.rs-1838 */ impl FiniteBitSetTy for u32 {
/* FP:bit_set.rs-1839 */     const DOMAIN_SIZE: u32 = 32;
/* FP:bit_set.rs-1840 */ 
/* FP:bit_set.rs-1841 */     const FILLED: Self = Self::MAX;
/* FP:bit_set.rs-1842 */     const EMPTY: Self = Self::MIN;
/* FP:bit_set.rs-1843 */ 
/* FP:bit_set.rs-1844 */     const ONE: Self = 1u32;
/* FP:bit_set.rs-1845 */     const ZERO: Self = 0u32;
/* FP:bit_set.rs-1846 */ 
/* FP:bit_set.rs-1847 */     fn checked_shl(self, rhs: u32) -> Option<Self> {
/* FP:bit_set.rs-1848 */         self.checked_shl(rhs)
/* FP:bit_set.rs-1849 */     }
/* FP:bit_set.rs-1850 */ 
/* FP:bit_set.rs-1851 */     fn checked_shr(self, rhs: u32) -> Option<Self> {
/* FP:bit_set.rs-1852 */         self.checked_shr(rhs)
/* FP:bit_set.rs-1853 */     }
/* FP:bit_set.rs-1854 */ }
/* FP:bit_set.rs-1855 */ 
/* FP:bit_set.rs-1856 */ impl std::fmt::Debug for FiniteBitSet<u32> {
/* FP:bit_set.rs-1857 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:bit_set.rs-1858 */         write!(f, "{:032b}", self.0)
/* FP:bit_set.rs-1859 */     }
/* FP:bit_set.rs-1860 */ }
/* FP:bit_set.rs-1861 */ 
/* FP:bit_set.rs-1862 */ /// A fixed-sized bitset type represented by an integer type. Indices outwith than the range
/* FP:bit_set.rs-1863 */ /// representable by `T` are considered set.
/* FP:bit_set.rs-1864 */ #[cfg_attr(feature = "nightly", derive(Decodable_NoContext, Encodable_NoContext))]
/* FP:bit_set.rs-1865 */ #[derive(Copy, Clone, Eq, PartialEq)]
/* FP:bit_set.rs-1866 */ pub struct FiniteBitSet<T: FiniteBitSetTy>(pub T);
/* FP:bit_set.rs-1867 */ 
/* FP:bit_set.rs-1868 */ impl<T: FiniteBitSetTy> FiniteBitSet<T> {
/* FP:bit_set.rs-1869 */     /// Creates a new, empty bitset.
/* FP:bit_set.rs-1870 */     pub fn new_empty() -> Self {
/* FP:bit_set.rs-1871 */         Self(T::EMPTY)
/* FP:bit_set.rs-1872 */     }
/* FP:bit_set.rs-1873 */ 
/* FP:bit_set.rs-1874 */     /// Sets the `index`th bit.
/* FP:bit_set.rs-1875 */     pub fn set(&mut self, index: u32) {
/* FP:bit_set.rs-1876 */         self.0 |= T::ONE.checked_shl(index).unwrap_or(T::ZERO);
/* FP:bit_set.rs-1877 */     }
/* FP:bit_set.rs-1878 */ 
/* FP:bit_set.rs-1879 */     /// Unsets the `index`th bit.
/* FP:bit_set.rs-1880 */     pub fn clear(&mut self, index: u32) {
/* FP:bit_set.rs-1881 */         self.0 &= !T::ONE.checked_shl(index).unwrap_or(T::ZERO);
/* FP:bit_set.rs-1882 */     }
/* FP:bit_set.rs-1883 */ 
/* FP:bit_set.rs-1884 */     /// Sets the `i`th to `j`th bits.
/* FP:bit_set.rs-1885 */     pub fn set_range(&mut self, range: Range<u32>) {
/* FP:bit_set.rs-1886 */         let bits = T::FILLED
/* FP:bit_set.rs-1887 */             .checked_shl(range.end - range.start)
/* FP:bit_set.rs-1888 */             .unwrap_or(T::ZERO)
/* FP:bit_set.rs-1889 */             .not()
/* FP:bit_set.rs-1890 */             .checked_shl(range.start)
/* FP:bit_set.rs-1891 */             .unwrap_or(T::ZERO);
/* FP:bit_set.rs-1892 */         self.0 |= bits;
/* FP:bit_set.rs-1893 */     }
/* FP:bit_set.rs-1894 */ 
/* FP:bit_set.rs-1895 */     /// Is the set empty?
/* FP:bit_set.rs-1896 */     pub fn is_empty(&self) -> bool {
/* FP:bit_set.rs-1897 */         self.0 == T::EMPTY
/* FP:bit_set.rs-1898 */     }
/* FP:bit_set.rs-1899 */ 
/* FP:bit_set.rs-1900 */     /// Returns the domain size of the bitset.
/* FP:bit_set.rs-1901 */     pub fn within_domain(&self, index: u32) -> bool {
/* FP:bit_set.rs-1902 */         index < T::DOMAIN_SIZE
/* FP:bit_set.rs-1903 */     }
/* FP:bit_set.rs-1904 */ 
/* FP:bit_set.rs-1905 */     /// Returns if the `index`th bit is set.
/* FP:bit_set.rs-1906 */     pub fn contains(&self, index: u32) -> Option<bool> {
/* FP:bit_set.rs-1907 */         self.within_domain(index)
/* FP:bit_set.rs-1908 */             .then(|| ((self.0.checked_shr(index).unwrap_or(T::ONE)) & T::ONE) == T::ONE)
/* FP:bit_set.rs-1909 */     }
/* FP:bit_set.rs-1910 */ }
/* FP:bit_set.rs-1911 */ 
/* FP:bit_set.rs-1912 */ impl<T: FiniteBitSetTy> Default for FiniteBitSet<T> {
/* FP:bit_set.rs-1913 */     fn default() -> Self {
/* FP:bit_set.rs-1914 */         Self::new_empty()
/* FP:bit_set.rs-1915 */     }
/* FP:bit_set.rs-1916 */ }