/* FP:interval.rs-0001 */ use std::iter::Step;
/* FP:interval.rs-0002 */ use std::marker::PhantomData;
/* FP:interval.rs-0003 */ use std::ops::{Bound, Range, RangeBounds};
/* FP:interval.rs-0004 */ 
/* FP:interval.rs-0005 */ use smallvec::SmallVec;
/* FP:interval.rs-0006 */ 
/* FP:interval.rs-0007 */ use crate::idx::Idx;
/* FP:interval.rs-0008 */ use crate::vec::IndexVec;
/* FP:interval.rs-0009 */ 
/* FP:interval.rs-0010 */ #[cfg(test)]
/* FP:interval.rs-0012 */ 
/* FP:interval.rs-0013 */ /// Stores a set of intervals on the indices.
/* FP:interval.rs-0014 */ ///
/* FP:interval.rs-0015 */ /// The elements in `map` are sorted and non-adjacent, which means
/* FP:interval.rs-0016 */ /// the second value of the previous element is *greater* than the
/* FP:interval.rs-0017 */ /// first value of the following element.
/* FP:interval.rs-0018 */ #[derive(Debug, Clone)]
/* FP:interval.rs-0019 */ pub struct IntervalSet<I> {
/* FP:interval.rs-0020 */     // Start, end (both inclusive)
/* FP:interval.rs-0021 */     map: SmallVec<[(u32, u32); 2]>,
/* FP:interval.rs-0022 */     domain: usize,
/* FP:interval.rs-0023 */     _data: PhantomData<I>,
/* FP:interval.rs-0024 */ }
/* FP:interval.rs-0025 */ 
/* FP:interval.rs-0026 */ #[inline]
/* FP:interval.rs-0027 */ fn inclusive_start<T: Idx>(range: impl RangeBounds<T>) -> u32 {
/* FP:interval.rs-0028 */     match range.start_bound() {
/* FP:interval.rs-0029 */         Bound::Included(start) => start.index() as u32,
/* FP:interval.rs-0030 */         Bound::Excluded(start) => start.index() as u32 + 1,
/* FP:interval.rs-0031 */         Bound::Unbounded => 0,
/* FP:interval.rs-0032 */     }
/* FP:interval.rs-0033 */ }
/* FP:interval.rs-0034 */ 
/* FP:interval.rs-0035 */ #[inline]
/* FP:interval.rs-0036 */ fn inclusive_end<T: Idx>(domain: usize, range: impl RangeBounds<T>) -> Option<u32> {
/* FP:interval.rs-0037 */     let end = match range.end_bound() {
/* FP:interval.rs-0038 */         Bound::Included(end) => end.index() as u32,
/* FP:interval.rs-0039 */         Bound::Excluded(end) => end.index().checked_sub(1)? as u32,
/* FP:interval.rs-0040 */         Bound::Unbounded => domain.checked_sub(1)? as u32,
/* FP:interval.rs-0041 */     };
/* FP:interval.rs-0042 */     Some(end)
/* FP:interval.rs-0043 */ }
/* FP:interval.rs-0044 */ 
/* FP:interval.rs-0045 */ impl<I: Idx> IntervalSet<I> {
/* FP:interval.rs-0046 */     pub fn new(domain: usize) -> IntervalSet<I> {
/* FP:interval.rs-0047 */         IntervalSet { map: SmallVec::new(), domain, _data: PhantomData }
/* FP:interval.rs-0048 */     }
/* FP:interval.rs-0049 */ 
/* FP:interval.rs-0050 */     pub fn clear(&mut self) {
/* FP:interval.rs-0051 */         self.map.clear();
/* FP:interval.rs-0052 */     }
/* FP:interval.rs-0053 */ 
/* FP:interval.rs-0054 */     pub fn iter(&self) -> impl Iterator<Item = I>
/* FP:interval.rs-0055 */     where
/* FP:interval.rs-0056 */         I: Step,
/* FP:interval.rs-0057 */     {
/* FP:interval.rs-0058 */         self.iter_intervals().flatten()
/* FP:interval.rs-0059 */     }
/* FP:interval.rs-0060 */ 
/* FP:interval.rs-0061 */     /// Iterates through intervals stored in the set, in order.
/* FP:interval.rs-0062 */     pub fn iter_intervals(&self) -> impl Iterator<Item = std::ops::Range<I>>
/* FP:interval.rs-0063 */     where
/* FP:interval.rs-0064 */         I: Step,
/* FP:interval.rs-0065 */     {
/* FP:interval.rs-0066 */         self.map.iter().map(|&(start, end)| I::new(start as usize)..I::new(end as usize + 1))
/* FP:interval.rs-0067 */     }
/* FP:interval.rs-0068 */ 
/* FP:interval.rs-0069 */     /// Returns true if we increased the number of elements present.
/* FP:interval.rs-0070 */     pub fn insert(&mut self, point: I) -> bool {
/* FP:interval.rs-0071 */         self.insert_range(point..=point)
/* FP:interval.rs-0072 */     }
/* FP:interval.rs-0073 */ 
/* FP:interval.rs-0074 */     /// Returns true if we increased the number of elements present.
/* FP:interval.rs-0075 */     pub fn insert_range(&mut self, range: impl RangeBounds<I> + Clone) -> bool {
/* FP:interval.rs-0076 */         let start = inclusive_start(range.clone());
/* FP:interval.rs-0077 */         let Some(end) = inclusive_end(self.domain, range) else {
/* FP:interval.rs-0078 */             // empty range
/* FP:interval.rs-0079 */             return false;
/* FP:interval.rs-0080 */         };
/* FP:interval.rs-0081 */         if start > end {
/* FP:interval.rs-0082 */             return false;
/* FP:interval.rs-0083 */         }
/* FP:interval.rs-0084 */ 
/* FP:interval.rs-0085 */         // This condition looks a bit weird, but actually makes sense.
/* FP:interval.rs-0086 */         //
/* FP:interval.rs-0087 */         // if r.0 == end + 1, then we're actually adjacent, so we want to
/* FP:interval.rs-0088 */         // continue to the next range. We're looking here for the first
/* FP:interval.rs-0089 */         // range which starts *non-adjacently* to our end.
/* FP:interval.rs-0090 */         let next = self.map.partition_point(|r| r.0 <= end + 1);
/* FP:interval.rs-0091 */         let result = if let Some(right) = next.checked_sub(1) {
/* FP:interval.rs-0092 */             let (prev_start, prev_end) = self.map[right];
/* FP:interval.rs-0093 */             if prev_end + 1 >= start {
/* FP:interval.rs-0094 */                 // If the start for the inserted range is adjacent to the
/* FP:interval.rs-0095 */                 // end of the previous, we can extend the previous range.
/* FP:interval.rs-0096 */                 if start < prev_start {
/* FP:interval.rs-0097 */                     // The first range which ends *non-adjacently* to our start.
/* FP:interval.rs-0098 */                     // And we can ensure that left <= right.
/* FP:interval.rs-0099 */                     let left = self.map.partition_point(|l| l.1 + 1 < start);
/* FP:interval.rs-0100 */                     let min = std::cmp::min(self.map[left].0, start);
/* FP:interval.rs-0101 */                     let max = std::cmp::max(prev_end, end);
/* FP:interval.rs-0102 */                     self.map[right] = (min, max);
/* FP:interval.rs-0103 */                     if left != right {
/* FP:interval.rs-0104 */                         self.map.drain(left..right);
/* FP:interval.rs-0105 */                     }
/* FP:interval.rs-0106 */                     true
/* FP:interval.rs-0107 */                 } else {
/* FP:interval.rs-0108 */                     // We overlap with the previous range, increase it to
/* FP:interval.rs-0109 */                     // include us.
/* FP:interval.rs-0110 */                     //
/* FP:interval.rs-0111 */                     // Make sure we're actually going to *increase* it though --
/* FP:interval.rs-0112 */                     // it may be that end is just inside the previously existing
/* FP:interval.rs-0113 */                     // set.
/* FP:interval.rs-0114 */                     if end > prev_end {
/* FP:interval.rs-0115 */                         self.map[right].1 = end;
/* FP:interval.rs-0116 */                         true
/* FP:interval.rs-0117 */                     } else {
/* FP:interval.rs-0118 */                         false
/* FP:interval.rs-0119 */                     }
/* FP:interval.rs-0120 */                 }
/* FP:interval.rs-0121 */             } else {
/* FP:interval.rs-0122 */                 // Otherwise, we don't overlap, so just insert
/* FP:interval.rs-0123 */                 self.map.insert(right + 1, (start, end));
/* FP:interval.rs-0124 */                 true
/* FP:interval.rs-0125 */             }
/* FP:interval.rs-0126 */         } else {
/* FP:interval.rs-0127 */             if self.map.is_empty() {
/* FP:interval.rs-0128 */                 // Quite common in practice, and expensive to call memcpy
/* FP:interval.rs-0129 */                 // with length zero.
/* FP:interval.rs-0130 */                 self.map.push((start, end));
/* FP:interval.rs-0131 */             } else {
/* FP:interval.rs-0132 */                 self.map.insert(next, (start, end));
/* FP:interval.rs-0133 */             }
/* FP:interval.rs-0134 */             true
/* FP:interval.rs-0135 */         };
/* FP:interval.rs-0136 */         debug_assert!(
/* FP:interval.rs-0137 */             self.check_invariants(),
/* FP:interval.rs-0138 */             "wrong intervals after insert {start:?}..={end:?} to {self:?}"
/* FP:interval.rs-0139 */         );
/* FP:interval.rs-0140 */         result
/* FP:interval.rs-0141 */     }
/* FP:interval.rs-0142 */ 
/* FP:interval.rs-0143 */     /// Specialized version of `insert` when we know that the inserted point is *after* any
/* FP:interval.rs-0144 */     /// contained.
/* FP:interval.rs-0145 */     pub fn append(&mut self, point: I) {
/* FP:interval.rs-0146 */         let point = point.index() as u32;
/* FP:interval.rs-0147 */ 
/* FP:interval.rs-0148 */         if let Some((_, last_end)) = self.map.last_mut() {
/* FP:interval.rs-0149 */             assert!(*last_end <= point);
/* FP:interval.rs-0150 */             if point == *last_end {
/* FP:interval.rs-0151 */                 // The point is already in the set.
/* FP:interval.rs-0152 */             } else if point == *last_end + 1 {
/* FP:interval.rs-0153 */                 *last_end = point;
/* FP:interval.rs-0154 */             } else {
/* FP:interval.rs-0155 */                 self.map.push((point, point));
/* FP:interval.rs-0156 */             }
/* FP:interval.rs-0157 */         } else {
/* FP:interval.rs-0158 */             self.map.push((point, point));
/* FP:interval.rs-0159 */         }
/* FP:interval.rs-0160 */ 
/* FP:interval.rs-0161 */         debug_assert!(
/* FP:interval.rs-0162 */             self.check_invariants(),
/* FP:interval.rs-0163 */             "wrong intervals after append {point:?} to {self:?}"
/* FP:interval.rs-0164 */         );
/* FP:interval.rs-0165 */     }
/* FP:interval.rs-0166 */ 
/* FP:interval.rs-0167 */     pub fn contains(&self, needle: I) -> bool {
/* FP:interval.rs-0168 */         let needle = needle.index() as u32;
/* FP:interval.rs-0169 */         let Some(last) = self.map.partition_point(|r| r.0 <= needle).checked_sub(1) else {
/* FP:interval.rs-0170 */             // All ranges in the map start after the new range's end
/* FP:interval.rs-0171 */             return false;
/* FP:interval.rs-0172 */         };
/* FP:interval.rs-0173 */         let (_, prev_end) = &self.map[last];
/* FP:interval.rs-0174 */         needle <= *prev_end
/* FP:interval.rs-0175 */     }
/* FP:interval.rs-0176 */ 
/* FP:interval.rs-0177 */     pub fn superset(&self, other: &IntervalSet<I>) -> bool
/* FP:interval.rs-0178 */     where
/* FP:interval.rs-0179 */         I: Step,
/* FP:interval.rs-0180 */     {
/* FP:interval.rs-0181 */         let mut sup_iter = self.iter_intervals();
/* FP:interval.rs-0182 */         let mut current = None;
/* FP:interval.rs-0183 */         let contains = |sup: Range<I>, sub: Range<I>, current: &mut Option<Range<I>>| {
/* FP:interval.rs-0184 */             if sup.end < sub.start {
/* FP:interval.rs-0185 */                 // if `sup.end == sub.start`, the next sup doesn't contain `sub.start`
/* FP:interval.rs-0186 */                 None // continue to the next sup
/* FP:interval.rs-0187 */             } else if sup.end >= sub.end && sup.start <= sub.start {
/* FP:interval.rs-0188 */                 *current = Some(sup); // save the current sup
/* FP:interval.rs-0189 */                 Some(true)
/* FP:interval.rs-0190 */             } else {
/* FP:interval.rs-0191 */                 Some(false)
/* FP:interval.rs-0192 */             }
/* FP:interval.rs-0193 */         };
/* FP:interval.rs-0194 */         other.iter_intervals().all(|sub| {
/* FP:interval.rs-0195 */             current
/* FP:interval.rs-0196 */                 .take()
/* FP:interval.rs-0197 */                 .and_then(|sup| contains(sup, sub.clone(), &mut current))
/* FP:interval.rs-0198 */                 .or_else(|| sup_iter.find_map(|sup| contains(sup, sub.clone(), &mut current)))
/* FP:interval.rs-0199 */                 .unwrap_or(false)
/* FP:interval.rs-0200 */         })
/* FP:interval.rs-0201 */     }
/* FP:interval.rs-0202 */ 
/* FP:interval.rs-0203 */     pub fn disjoint(&self, other: &IntervalSet<I>) -> bool
/* FP:interval.rs-0204 */     where
/* FP:interval.rs-0205 */         I: Step,
/* FP:interval.rs-0206 */     {
/* FP:interval.rs-0207 */         let helper = move || {
/* FP:interval.rs-0208 */             let mut self_iter = self.iter_intervals();
/* FP:interval.rs-0209 */             let mut other_iter = other.iter_intervals();
/* FP:interval.rs-0210 */ 
/* FP:interval.rs-0211 */             let mut self_current = self_iter.next()?;
/* FP:interval.rs-0212 */             let mut other_current = other_iter.next()?;
/* FP:interval.rs-0213 */ 
/* FP:interval.rs-0214 */             loop {
/* FP:interval.rs-0215 */                 if self_current.end <= other_current.start {
/* FP:interval.rs-0216 */                     self_current = self_iter.next()?;
/* FP:interval.rs-0217 */                     continue;
/* FP:interval.rs-0218 */                 }
/* FP:interval.rs-0219 */                 if other_current.end <= self_current.start {
/* FP:interval.rs-0220 */                     other_current = other_iter.next()?;
/* FP:interval.rs-0221 */                     continue;
/* FP:interval.rs-0222 */                 }
/* FP:interval.rs-0223 */                 return Some(false);
/* FP:interval.rs-0224 */             }
/* FP:interval.rs-0225 */         };
/* FP:interval.rs-0226 */         helper().unwrap_or(true)
/* FP:interval.rs-0227 */     }
/* FP:interval.rs-0228 */ 
/* FP:interval.rs-0229 */     pub fn is_empty(&self) -> bool {
/* FP:interval.rs-0230 */         self.map.is_empty()
/* FP:interval.rs-0231 */     }
/* FP:interval.rs-0232 */ 
/* FP:interval.rs-0233 */     /// Equivalent to `range.iter().find(|i| !self.contains(i))`.
/* FP:interval.rs-0234 */     pub fn first_unset_in(&self, range: impl RangeBounds<I> + Clone) -> Option<I> {
/* FP:interval.rs-0235 */         let start = inclusive_start(range.clone());
/* FP:interval.rs-0236 */         let Some(end) = inclusive_end(self.domain, range) else {
/* FP:interval.rs-0237 */             // empty range
/* FP:interval.rs-0238 */             return None;
/* FP:interval.rs-0239 */         };
/* FP:interval.rs-0240 */         if start > end {
/* FP:interval.rs-0241 */             return None;
/* FP:interval.rs-0242 */         }
/* FP:interval.rs-0243 */         let Some(last) = self.map.partition_point(|r| r.0 <= start).checked_sub(1) else {
/* FP:interval.rs-0244 */             // All ranges in the map start after the new range's end
/* FP:interval.rs-0245 */             return Some(I::new(start as usize));
/* FP:interval.rs-0246 */         };
/* FP:interval.rs-0247 */         let (_, prev_end) = self.map[last];
/* FP:interval.rs-0248 */         if start > prev_end {
/* FP:interval.rs-0249 */             Some(I::new(start as usize))
/* FP:interval.rs-0250 */         } else if prev_end < end {
/* FP:interval.rs-0251 */             Some(I::new(prev_end as usize + 1))
/* FP:interval.rs-0252 */         } else {
/* FP:interval.rs-0253 */             None
/* FP:interval.rs-0254 */         }
/* FP:interval.rs-0255 */     }
/* FP:interval.rs-0256 */ 
/* FP:interval.rs-0257 */     /// Returns the maximum (last) element present in the set from `range`.
/* FP:interval.rs-0258 */     pub fn last_set_in(&self, range: impl RangeBounds<I> + Clone) -> Option<I> {
/* FP:interval.rs-0259 */         let start = inclusive_start(range.clone());
/* FP:interval.rs-0260 */         let Some(end) = inclusive_end(self.domain, range) else {
/* FP:interval.rs-0261 */             // empty range
/* FP:interval.rs-0262 */             return None;
/* FP:interval.rs-0263 */         };
/* FP:interval.rs-0264 */         if start > end {
/* FP:interval.rs-0265 */             return None;
/* FP:interval.rs-0266 */         }
/* FP:interval.rs-0267 */         let Some(last) = self.map.partition_point(|r| r.0 <= end).checked_sub(1) else {
/* FP:interval.rs-0268 */             // All ranges in the map start after the new range's end
/* FP:interval.rs-0269 */             return None;
/* FP:interval.rs-0270 */         };
/* FP:interval.rs-0271 */         let (_, prev_end) = &self.map[last];
/* FP:interval.rs-0272 */         if start <= *prev_end { Some(I::new(std::cmp::min(*prev_end, end) as usize)) } else { None }
/* FP:interval.rs-0273 */     }
/* FP:interval.rs-0274 */ 
/* FP:interval.rs-0275 */     pub fn insert_all(&mut self) {
/* FP:interval.rs-0276 */         self.clear();
/* FP:interval.rs-0277 */         if let Some(end) = self.domain.checked_sub(1) {
/* FP:interval.rs-0278 */             self.map.push((0, end.try_into().unwrap()));
/* FP:interval.rs-0279 */         }
/* FP:interval.rs-0280 */         debug_assert!(self.check_invariants());
/* FP:interval.rs-0281 */     }
/* FP:interval.rs-0282 */ 
/* FP:interval.rs-0283 */     pub fn union(&mut self, other: &IntervalSet<I>) -> bool
/* FP:interval.rs-0284 */     where
/* FP:interval.rs-0285 */         I: Step,
/* FP:interval.rs-0286 */     {
/* FP:interval.rs-0287 */         assert_eq!(self.domain, other.domain);
/* FP:interval.rs-0288 */         if self.map.len() < other.map.len() {
/* FP:interval.rs-0289 */             let backup = self.clone();
/* FP:interval.rs-0290 */             self.map.clone_from(&other.map);
/* FP:interval.rs-0291 */             return self.union(&backup);
/* FP:interval.rs-0292 */         }
/* FP:interval.rs-0293 */ 
/* FP:interval.rs-0294 */         let mut did_insert = false;
/* FP:interval.rs-0295 */         for range in other.iter_intervals() {
/* FP:interval.rs-0296 */             did_insert |= self.insert_range(range);
/* FP:interval.rs-0297 */         }
/* FP:interval.rs-0298 */         debug_assert!(self.check_invariants());
/* FP:interval.rs-0299 */         did_insert
/* FP:interval.rs-0300 */     }
/* FP:interval.rs-0301 */ 
/* FP:interval.rs-0302 */     // Check the intervals are valid, sorted and non-adjacent
/* FP:interval.rs-0303 */     fn check_invariants(&self) -> bool {
/* FP:interval.rs-0304 */         let mut current: Option<u32> = None;
/* FP:interval.rs-0305 */         for (start, end) in &self.map {
/* FP:interval.rs-0306 */             if start > end || current.is_some_and(|x| x + 1 >= *start) {
/* FP:interval.rs-0307 */                 return false;
/* FP:interval.rs-0308 */             }
/* FP:interval.rs-0309 */             current = Some(*end);
/* FP:interval.rs-0310 */         }
/* FP:interval.rs-0311 */         current.is_none_or(|x| x < self.domain as u32)
/* FP:interval.rs-0312 */     }
/* FP:interval.rs-0313 */ }
/* FP:interval.rs-0314 */ 
/* FP:interval.rs-0315 */ /// This data structure optimizes for cases where the stored bits in each row
/* FP:interval.rs-0316 */ /// are expected to be highly contiguous (long ranges of 1s or 0s), in contrast
/* FP:interval.rs-0317 */ /// to BitMatrix and SparseBitMatrix which are optimized for
/* FP:interval.rs-0318 */ /// "random"/non-contiguous bits and cheap(er) point queries at the expense of
/* FP:interval.rs-0319 */ /// memory usage.
/* FP:interval.rs-0320 */ #[derive(Clone)]
/* FP:interval.rs-0321 */ pub struct SparseIntervalMatrix<R, C>
/* FP:interval.rs-0322 */ where
/* FP:interval.rs-0323 */     R: Idx,
/* FP:interval.rs-0324 */     C: Idx,
/* FP:interval.rs-0325 */ {
/* FP:interval.rs-0326 */     rows: IndexVec<R, IntervalSet<C>>,
/* FP:interval.rs-0327 */     column_size: usize,
/* FP:interval.rs-0328 */ }
/* FP:interval.rs-0329 */ 
/* FP:interval.rs-0330 */ impl<R: Idx, C: Step + Idx> SparseIntervalMatrix<R, C> {
/* FP:interval.rs-0331 */     pub fn new(column_size: usize) -> SparseIntervalMatrix<R, C> {
/* FP:interval.rs-0332 */         SparseIntervalMatrix { rows: IndexVec::new(), column_size }
/* FP:interval.rs-0333 */     }
/* FP:interval.rs-0334 */ 
/* FP:interval.rs-0335 */     pub fn rows(&self) -> impl Iterator<Item = R> {
/* FP:interval.rs-0336 */         self.rows.indices()
/* FP:interval.rs-0337 */     }
/* FP:interval.rs-0338 */ 
/* FP:interval.rs-0339 */     pub fn row(&self, row: R) -> Option<&IntervalSet<C>> {
/* FP:interval.rs-0340 */         self.rows.get(row)
/* FP:interval.rs-0341 */     }
/* FP:interval.rs-0342 */ 
/* FP:interval.rs-0343 */     fn ensure_row(&mut self, row: R) -> &mut IntervalSet<C> {
/* FP:interval.rs-0344 */         self.rows.ensure_contains_elem(row, || IntervalSet::new(self.column_size))
/* FP:interval.rs-0345 */     }
/* FP:interval.rs-0346 */ 
/* FP:interval.rs-0347 */     pub fn union_row(&mut self, row: R, from: &IntervalSet<C>) -> bool
/* FP:interval.rs-0348 */     where
/* FP:interval.rs-0349 */         C: Step,
/* FP:interval.rs-0350 */     {
/* FP:interval.rs-0351 */         self.ensure_row(row).union(from)
/* FP:interval.rs-0352 */     }
/* FP:interval.rs-0353 */ 
/* FP:interval.rs-0354 */     pub fn union_rows(&mut self, read: R, write: R) -> bool
/* FP:interval.rs-0355 */     where
/* FP:interval.rs-0356 */         C: Step,
/* FP:interval.rs-0357 */     {
/* FP:interval.rs-0358 */         if read == write || self.rows.get(read).is_none() {
/* FP:interval.rs-0359 */             return false;
/* FP:interval.rs-0360 */         }
/* FP:interval.rs-0361 */         self.ensure_row(write);
/* FP:interval.rs-0362 */         let (read_row, write_row) = self.rows.pick2_mut(read, write);
/* FP:interval.rs-0363 */         write_row.union(read_row)
/* FP:interval.rs-0364 */     }
/* FP:interval.rs-0365 */ 
/* FP:interval.rs-0366 */     pub fn insert_all_into_row(&mut self, row: R) {
/* FP:interval.rs-0367 */         self.ensure_row(row).insert_all();
/* FP:interval.rs-0368 */     }
/* FP:interval.rs-0369 */ 
/* FP:interval.rs-0370 */     pub fn insert_range(&mut self, row: R, range: impl RangeBounds<C> + Clone) {
/* FP:interval.rs-0371 */         self.ensure_row(row).insert_range(range);
/* FP:interval.rs-0372 */     }
/* FP:interval.rs-0373 */ 
/* FP:interval.rs-0374 */     pub fn insert(&mut self, row: R, point: C) -> bool {
/* FP:interval.rs-0375 */         self.ensure_row(row).insert(point)
/* FP:interval.rs-0376 */     }
/* FP:interval.rs-0377 */ 
/* FP:interval.rs-0378 */     pub fn append(&mut self, row: R, point: C) {
/* FP:interval.rs-0379 */         self.ensure_row(row).append(point)
/* FP:interval.rs-0380 */     }
/* FP:interval.rs-0381 */ 
/* FP:interval.rs-0382 */     pub fn contains(&self, row: R, point: C) -> bool {
/* FP:interval.rs-0383 */         self.row(row).is_some_and(|r| r.contains(point))
/* FP:interval.rs-0384 */     }
/* FP:interval.rs-0385 */ }