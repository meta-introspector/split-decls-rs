/* FP:edit_distance.rs-0001 */ // Edit distances.
/* FP:edit_distance.rs-0002 */ //
/* FP:edit_distance.rs-0003 */ // The [edit distance] is a metric for measuring the difference between two strings.
/* FP:edit_distance.rs-0004 */ //
/* FP:edit_distance.rs-0005 */ // [edit distance]: https://en.wikipedia.org/wiki/Edit_distance
/* FP:edit_distance.rs-0006 */ 
/* FP:edit_distance.rs-0007 */ // The current implementation is the restricted Damerau-Levenshtein algorithm. It is restricted
/* FP:edit_distance.rs-0008 */ // because it does not permit modifying characters that have already been transposed. The specific
/* FP:edit_distance.rs-0009 */ // algorithm should not matter to the caller of the methods, which is why it is not noted in the
/* FP:edit_distance.rs-0010 */ // documentation.
/* FP:edit_distance.rs-0011 */ 
/* FP:edit_distance.rs-0012 */ use std::{cmp, mem};
/* FP:edit_distance.rs-0013 */ 
/* FP:edit_distance.rs-0014 */ use crate::Symbol;
/* FP:edit_distance.rs-0015 */ 
/* FP:edit_distance.rs-0016 */ #[cfg(test)]
/* FP:edit_distance.rs-0018 */ 
/* FP:edit_distance.rs-0019 */ /// Finds the [edit distance] between two strings.
/* FP:edit_distance.rs-0020 */ ///
/* FP:edit_distance.rs-0021 */ /// Returns `None` if the distance exceeds the limit.
/* FP:edit_distance.rs-0022 */ ///
/* FP:edit_distance.rs-0023 */ /// [edit distance]: https://en.wikipedia.org/wiki/Edit_distance
/* FP:edit_distance.rs-0024 */ pub fn edit_distance(a: &str, b: &str, limit: usize) -> Option<usize> {
/* FP:edit_distance.rs-0025 */     let mut a = &a.chars().collect::<Vec<_>>()[..];
/* FP:edit_distance.rs-0026 */     let mut b = &b.chars().collect::<Vec<_>>()[..];
/* FP:edit_distance.rs-0027 */ 
/* FP:edit_distance.rs-0028 */     // Ensure that `b` is the shorter string, minimizing memory use.
/* FP:edit_distance.rs-0029 */     if a.len() < b.len() {
/* FP:edit_distance.rs-0030 */         mem::swap(&mut a, &mut b);
/* FP:edit_distance.rs-0031 */     }
/* FP:edit_distance.rs-0032 */ 
/* FP:edit_distance.rs-0033 */     let min_dist = a.len() - b.len();
/* FP:edit_distance.rs-0034 */     // If we know the limit will be exceeded, we can return early.
/* FP:edit_distance.rs-0035 */     if min_dist > limit {
/* FP:edit_distance.rs-0036 */         return None;
/* FP:edit_distance.rs-0037 */     }
/* FP:edit_distance.rs-0038 */ 
/* FP:edit_distance.rs-0039 */     // Strip common prefix.
/* FP:edit_distance.rs-0040 */     while let Some(((b_char, b_rest), (a_char, a_rest))) = b.split_first().zip(a.split_first())
/* FP:edit_distance.rs-0041 */         && a_char == b_char
/* FP:edit_distance.rs-0042 */     {
/* FP:edit_distance.rs-0043 */         a = a_rest;
/* FP:edit_distance.rs-0044 */         b = b_rest;
/* FP:edit_distance.rs-0045 */     }
/* FP:edit_distance.rs-0046 */     // Strip common suffix.
/* FP:edit_distance.rs-0047 */     while let Some(((b_char, b_rest), (a_char, a_rest))) = b.split_last().zip(a.split_last())
/* FP:edit_distance.rs-0048 */         && a_char == b_char
/* FP:edit_distance.rs-0049 */     {
/* FP:edit_distance.rs-0050 */         a = a_rest;
/* FP:edit_distance.rs-0051 */         b = b_rest;
/* FP:edit_distance.rs-0052 */     }
/* FP:edit_distance.rs-0053 */ 
/* FP:edit_distance.rs-0054 */     // If either string is empty, the distance is the length of the other.
/* FP:edit_distance.rs-0055 */     // We know that `b` is the shorter string, so we don't need to check `a`.
/* FP:edit_distance.rs-0056 */     if b.len() == 0 {
/* FP:edit_distance.rs-0057 */         return Some(min_dist);
/* FP:edit_distance.rs-0058 */     }
/* FP:edit_distance.rs-0059 */ 
/* FP:edit_distance.rs-0060 */     let mut prev_prev = vec![usize::MAX; b.len() + 1];
/* FP:edit_distance.rs-0061 */     let mut prev = (0..=b.len()).collect::<Vec<_>>();
/* FP:edit_distance.rs-0062 */     let mut current = vec![0; b.len() + 1];
/* FP:edit_distance.rs-0063 */ 
/* FP:edit_distance.rs-0064 */     // row by row
/* FP:edit_distance.rs-0065 */     for i in 1..=a.len() {
/* FP:edit_distance.rs-0066 */         current[0] = i;
/* FP:edit_distance.rs-0067 */         let a_idx = i - 1;
/* FP:edit_distance.rs-0068 */ 
/* FP:edit_distance.rs-0069 */         // column by column
/* FP:edit_distance.rs-0070 */         for j in 1..=b.len() {
/* FP:edit_distance.rs-0071 */             let b_idx = j - 1;
/* FP:edit_distance.rs-0072 */ 
/* FP:edit_distance.rs-0073 */             // There is no cost to substitute a character with itself.
/* FP:edit_distance.rs-0074 */             let substitution_cost = if a[a_idx] == b[b_idx] { 0 } else { 1 };
/* FP:edit_distance.rs-0075 */ 
/* FP:edit_distance.rs-0076 */             current[j] = cmp::min(
/* FP:edit_distance.rs-0077 */                 // deletion
/* FP:edit_distance.rs-0078 */                 prev[j] + 1,
/* FP:edit_distance.rs-0079 */                 cmp::min(
/* FP:edit_distance.rs-0080 */                     // insertion
/* FP:edit_distance.rs-0081 */                     current[j - 1] + 1,
/* FP:edit_distance.rs-0082 */                     // substitution
/* FP:edit_distance.rs-0083 */                     prev[j - 1] + substitution_cost,
/* FP:edit_distance.rs-0084 */                 ),
/* FP:edit_distance.rs-0085 */             );
/* FP:edit_distance.rs-0086 */ 
/* FP:edit_distance.rs-0087 */             if (i > 1) && (j > 1) && (a[a_idx] == b[b_idx - 1]) && (a[a_idx - 1] == b[b_idx]) {
/* FP:edit_distance.rs-0088 */                 // transposition
/* FP:edit_distance.rs-0089 */                 current[j] = cmp::min(current[j], prev_prev[j - 2] + 1);
/* FP:edit_distance.rs-0090 */             }
/* FP:edit_distance.rs-0091 */         }
/* FP:edit_distance.rs-0092 */ 
/* FP:edit_distance.rs-0093 */         // Rotate the buffers, reusing the memory.
/* FP:edit_distance.rs-0094 */         [prev_prev, prev, current] = [prev, current, prev_prev];
/* FP:edit_distance.rs-0095 */     }
/* FP:edit_distance.rs-0096 */ 
/* FP:edit_distance.rs-0097 */     // `prev` because we already rotated the buffers.
/* FP:edit_distance.rs-0098 */     let distance = prev[b.len()];
/* FP:edit_distance.rs-0099 */     (distance <= limit).then_some(distance)
/* FP:edit_distance.rs-0100 */ }
/* FP:edit_distance.rs-0101 */ 
/* FP:edit_distance.rs-0102 */ /// Provides a word similarity score between two words that accounts for substrings being more
/* FP:edit_distance.rs-0103 */ /// meaningful than a typical edit distance. The lower the score, the closer the match. 0 is an
/* FP:edit_distance.rs-0104 */ /// identical match.
/* FP:edit_distance.rs-0105 */ ///
/* FP:edit_distance.rs-0106 */ /// Uses the edit distance between the two strings and removes the cost of the length difference.
/* FP:edit_distance.rs-0107 */ /// If this is 0 then it is either a substring match or a full word match, in the substring match
/* FP:edit_distance.rs-0108 */ /// case we detect this and return `1`. To prevent finding meaningless substrings, eg. "in" in
/* FP:edit_distance.rs-0109 */ /// "shrink", we only perform this subtraction of length difference if one of the words is not
/* FP:edit_distance.rs-0110 */ /// greater than twice the length of the other. For cases where the words are close in size but not
/* FP:edit_distance.rs-0111 */ /// an exact substring then the cost of the length difference is discounted by half.
/* FP:edit_distance.rs-0112 */ ///
/* FP:edit_distance.rs-0113 */ /// Returns `None` if the distance exceeds the limit.
/* FP:edit_distance.rs-0114 */ pub fn edit_distance_with_substrings(a: &str, b: &str, limit: usize) -> Option<usize> {
/* FP:edit_distance.rs-0115 */     let n = a.chars().count();
/* FP:edit_distance.rs-0116 */     let m = b.chars().count();
/* FP:edit_distance.rs-0117 */ 
/* FP:edit_distance.rs-0118 */     // Check one isn't less than half the length of the other. If this is true then there is a
/* FP:edit_distance.rs-0119 */     // big difference in length.
/* FP:edit_distance.rs-0120 */     let big_len_diff = (n * 2) < m || (m * 2) < n;
/* FP:edit_distance.rs-0121 */     let len_diff = m.abs_diff(n);
/* FP:edit_distance.rs-0122 */     let distance = edit_distance(a, b, limit + len_diff)?;
/* FP:edit_distance.rs-0123 */ 
/* FP:edit_distance.rs-0124 */     // This is the crux, subtracting length difference means exact substring matches will now be 0
/* FP:edit_distance.rs-0125 */     let score = distance - len_diff;
/* FP:edit_distance.rs-0126 */ 
/* FP:edit_distance.rs-0127 */     // If the score is 0 but the words have different lengths then it's a substring match not a full
/* FP:edit_distance.rs-0128 */     // word match
/* FP:edit_distance.rs-0129 */     let score = if score == 0 && len_diff > 0 && !big_len_diff {
/* FP:edit_distance.rs-0130 */         1 // Exact substring match, but not a total word match so return non-zero
/* FP:edit_distance.rs-0131 */     } else if !big_len_diff {
/* FP:edit_distance.rs-0132 */         // Not a big difference in length, discount cost of length difference
/* FP:edit_distance.rs-0133 */         score + len_diff.div_ceil(2)
/* FP:edit_distance.rs-0134 */     } else {
/* FP:edit_distance.rs-0135 */         // A big difference in length, add back the difference in length to the score
/* FP:edit_distance.rs-0136 */         score + len_diff
/* FP:edit_distance.rs-0137 */     };
/* FP:edit_distance.rs-0138 */ 
/* FP:edit_distance.rs-0139 */     (score <= limit).then_some(score)
/* FP:edit_distance.rs-0140 */ }
/* FP:edit_distance.rs-0141 */ 
/* FP:edit_distance.rs-0142 */ /// Finds the best match for given word in the given iterator where substrings are meaningful.
/* FP:edit_distance.rs-0143 */ ///
/* FP:edit_distance.rs-0144 */ /// A version of [`find_best_match_for_name`] that uses [`edit_distance_with_substrings`] as the
/* FP:edit_distance.rs-0145 */ /// score for word similarity. This takes an optional distance limit which defaults to one-third of
/* FP:edit_distance.rs-0146 */ /// the given word.
/* FP:edit_distance.rs-0147 */ ///
/* FP:edit_distance.rs-0148 */ /// We use case insensitive comparison to improve accuracy on an edge case with a lower(upper)case
/* FP:edit_distance.rs-0149 */ /// letters mismatch.
/* FP:edit_distance.rs-0150 */ pub fn find_best_match_for_name_with_substrings(
/* FP:edit_distance.rs-0151 */     candidates: &[Symbol],
/* FP:edit_distance.rs-0152 */     lookup: Symbol,
/* FP:edit_distance.rs-0153 */     dist: Option<usize>,
/* FP:edit_distance.rs-0154 */ ) -> Option<Symbol> {
/* FP:edit_distance.rs-0155 */     find_best_match_for_name_impl(true, candidates, lookup, dist)
/* FP:edit_distance.rs-0156 */ }
/* FP:edit_distance.rs-0157 */ 
/* FP:edit_distance.rs-0158 */ /// Finds the best match for a given word in the given iterator.
/* FP:edit_distance.rs-0159 */ ///
/* FP:edit_distance.rs-0160 */ /// As a loose rule to avoid the obviously incorrect suggestions, it takes
/* FP:edit_distance.rs-0161 */ /// an optional limit for the maximum allowable edit distance, which defaults
/* FP:edit_distance.rs-0162 */ /// to one-third of the given word.
/* FP:edit_distance.rs-0163 */ ///
/* FP:edit_distance.rs-0164 */ /// We use case insensitive comparison to improve accuracy on an edge case with a lower(upper)case
/* FP:edit_distance.rs-0165 */ /// letters mismatch.
/* FP:edit_distance.rs-0166 */ pub fn find_best_match_for_name(
/* FP:edit_distance.rs-0167 */     candidates: &[Symbol],
/* FP:edit_distance.rs-0168 */     lookup: Symbol,
/* FP:edit_distance.rs-0169 */     dist: Option<usize>,
/* FP:edit_distance.rs-0170 */ ) -> Option<Symbol> {
/* FP:edit_distance.rs-0171 */     find_best_match_for_name_impl(false, candidates, lookup, dist)
/* FP:edit_distance.rs-0172 */ }
/* FP:edit_distance.rs-0173 */ 
/* FP:edit_distance.rs-0174 */ /// Find the best match for multiple words
/* FP:edit_distance.rs-0175 */ ///
/* FP:edit_distance.rs-0176 */ /// This function is intended for use when the desired match would never be
/* FP:edit_distance.rs-0177 */ /// returned due to a substring in `lookup` which is superfluous.
/* FP:edit_distance.rs-0178 */ ///
/* FP:edit_distance.rs-0179 */ /// For example, when looking for the closest lint name to `clippy:missing_docs`,
/* FP:edit_distance.rs-0180 */ /// we would find `clippy::erasing_op`, despite `missing_docs` existing and being a better suggestion.
/* FP:edit_distance.rs-0181 */ /// `missing_docs` would have a larger edit distance because it does not contain the `clippy` tool prefix.
/* FP:edit_distance.rs-0182 */ /// In order to find `missing_docs`, this function takes multiple lookup strings, computes the best match
/* FP:edit_distance.rs-0183 */ /// for each and returns the match which had the lowest edit distance. In our example, `clippy:missing_docs` and
/* FP:edit_distance.rs-0184 */ /// `missing_docs` would be `lookups`, enabling `missing_docs` to be the best match, as desired.
/* FP:edit_distance.rs-0185 */ pub fn find_best_match_for_names(
/* FP:edit_distance.rs-0186 */     candidates: &[Symbol],
/* FP:edit_distance.rs-0187 */     lookups: &[Symbol],
/* FP:edit_distance.rs-0188 */     dist: Option<usize>,
/* FP:edit_distance.rs-0189 */ ) -> Option<Symbol> {
/* FP:edit_distance.rs-0190 */     lookups
/* FP:edit_distance.rs-0191 */         .iter()
/* FP:edit_distance.rs-0192 */         .map(|s| (s, find_best_match_for_name_impl(false, candidates, *s, dist)))
/* FP:edit_distance.rs-0193 */         .filter_map(|(s, r)| r.map(|r| (s, r)))
/* FP:edit_distance.rs-0194 */         .min_by(|(s1, r1), (s2, r2)| {
/* FP:edit_distance.rs-0195 */             let d1 = edit_distance(s1.as_str(), r1.as_str(), usize::MAX).unwrap();
/* FP:edit_distance.rs-0196 */             let d2 = edit_distance(s2.as_str(), r2.as_str(), usize::MAX).unwrap();
/* FP:edit_distance.rs-0197 */             d1.cmp(&d2)
/* FP:edit_distance.rs-0198 */         })
/* FP:edit_distance.rs-0199 */         .map(|(_, r)| r)
/* FP:edit_distance.rs-0200 */ }
/* FP:edit_distance.rs-0201 */ 
/* FP:edit_distance.rs-0202 */ #[cold]
/* FP:edit_distance.rs-0203 */ fn find_best_match_for_name_impl(
/* FP:edit_distance.rs-0204 */     use_substring_score: bool,
/* FP:edit_distance.rs-0205 */     candidates: &[Symbol],
/* FP:edit_distance.rs-0206 */     lookup_symbol: Symbol,
/* FP:edit_distance.rs-0207 */     dist: Option<usize>,
/* FP:edit_distance.rs-0208 */ ) -> Option<Symbol> {
/* FP:edit_distance.rs-0209 */     let lookup = lookup_symbol.as_str();
/* FP:edit_distance.rs-0210 */     let lookup_uppercase = lookup.to_uppercase();
/* FP:edit_distance.rs-0211 */ 
/* FP:edit_distance.rs-0212 */     // Priority of matches:
/* FP:edit_distance.rs-0213 */     // 1. Exact case insensitive match
/* FP:edit_distance.rs-0214 */     // 2. Edit distance match
/* FP:edit_distance.rs-0215 */     // 3. Sorted word match
/* FP:edit_distance.rs-0216 */     if let Some(c) = candidates.iter().find(|c| c.as_str().to_uppercase() == lookup_uppercase) {
/* FP:edit_distance.rs-0217 */         return Some(*c);
/* FP:edit_distance.rs-0218 */     }
/* FP:edit_distance.rs-0219 */ 
/* FP:edit_distance.rs-0220 */     // `fn edit_distance()` use `chars()` to calculate edit distance, so we must
/* FP:edit_distance.rs-0221 */     // also use `chars()` (and not `str::len()`) to calculate length here.
/* FP:edit_distance.rs-0222 */     let lookup_len = lookup.chars().count();
/* FP:edit_distance.rs-0223 */ 
/* FP:edit_distance.rs-0224 */     let mut dist = dist.unwrap_or_else(|| cmp::max(lookup_len, 3) / 3);
/* FP:edit_distance.rs-0225 */     let mut best = None;
/* FP:edit_distance.rs-0226 */     // store the candidates with the same distance, only for `use_substring_score` current.
/* FP:edit_distance.rs-0227 */     let mut next_candidates = vec![];
/* FP:edit_distance.rs-0228 */     for c in candidates {
/* FP:edit_distance.rs-0229 */         match if use_substring_score {
/* FP:edit_distance.rs-0230 */             edit_distance_with_substrings(lookup, c.as_str(), dist)
/* FP:edit_distance.rs-0231 */         } else {
/* FP:edit_distance.rs-0232 */             edit_distance(lookup, c.as_str(), dist)
/* FP:edit_distance.rs-0233 */         } {
/* FP:edit_distance.rs-0234 */             Some(0) => return Some(*c),
/* FP:edit_distance.rs-0235 */             Some(d) => {
/* FP:edit_distance.rs-0236 */                 if use_substring_score {
/* FP:edit_distance.rs-0237 */                     if d < dist {
/* FP:edit_distance.rs-0238 */                         dist = d;
/* FP:edit_distance.rs-0239 */                         next_candidates.clear();
/* FP:edit_distance.rs-0240 */                     } else {
/* FP:edit_distance.rs-0241 */                         // `d == dist` here, we need to store the candidates with the same distance
/* FP:edit_distance.rs-0242 */                         // so we won't decrease the distance in the next loop.
/* FP:edit_distance.rs-0243 */                     }
/* FP:edit_distance.rs-0244 */                     next_candidates.push(*c);
/* FP:edit_distance.rs-0245 */                 } else {
/* FP:edit_distance.rs-0246 */                     dist = d - 1;
/* FP:edit_distance.rs-0247 */                 }
/* FP:edit_distance.rs-0248 */                 best = Some(*c);
/* FP:edit_distance.rs-0249 */             }
/* FP:edit_distance.rs-0250 */             None => {}
/* FP:edit_distance.rs-0251 */         }
/* FP:edit_distance.rs-0252 */     }
/* FP:edit_distance.rs-0253 */ 
/* FP:edit_distance.rs-0254 */     // We have a tie among several candidates, try to select the best among them ignoring substrings.
/* FP:edit_distance.rs-0255 */     // For example, the candidates list `force_capture`, `capture`, and user inputted `forced_capture`,
/* FP:edit_distance.rs-0256 */     // we select `force_capture` with a extra round of edit distance calculation.
/* FP:edit_distance.rs-0257 */     if next_candidates.len() > 1 {
/* FP:edit_distance.rs-0258 */         debug_assert!(use_substring_score);
/* FP:edit_distance.rs-0259 */         best = find_best_match_for_name_impl(
/* FP:edit_distance.rs-0260 */             false,
/* FP:edit_distance.rs-0261 */             &next_candidates,
/* FP:edit_distance.rs-0262 */             lookup_symbol,
/* FP:edit_distance.rs-0263 */             Some(lookup.len()),
/* FP:edit_distance.rs-0264 */         );
/* FP:edit_distance.rs-0265 */     }
/* FP:edit_distance.rs-0266 */     if best.is_some() {
/* FP:edit_distance.rs-0267 */         return best;
/* FP:edit_distance.rs-0268 */     }
/* FP:edit_distance.rs-0269 */ 
/* FP:edit_distance.rs-0270 */     find_match_by_sorted_words(candidates, lookup)
/* FP:edit_distance.rs-0271 */ }
/* FP:edit_distance.rs-0272 */ 
/* FP:edit_distance.rs-0273 */ fn find_match_by_sorted_words(iter_names: &[Symbol], lookup: &str) -> Option<Symbol> {
/* FP:edit_distance.rs-0274 */     let lookup_sorted_by_words = sort_by_words(lookup);
/* FP:edit_distance.rs-0275 */     iter_names.iter().fold(None, |result, candidate| {
/* FP:edit_distance.rs-0276 */         if sort_by_words(candidate.as_str()) == lookup_sorted_by_words {
/* FP:edit_distance.rs-0277 */             Some(*candidate)
/* FP:edit_distance.rs-0278 */         } else {
/* FP:edit_distance.rs-0279 */             result
/* FP:edit_distance.rs-0280 */         }
/* FP:edit_distance.rs-0281 */     })
/* FP:edit_distance.rs-0282 */ }
/* FP:edit_distance.rs-0283 */ 
/* FP:edit_distance.rs-0284 */ fn sort_by_words(name: &str) -> Vec<&str> {
/* FP:edit_distance.rs-0285 */     let mut split_words: Vec<&str> = name.split('_').collect();
/* FP:edit_distance.rs-0286 */     // We are sorting primitive &strs and can use unstable sort here.
/* FP:edit_distance.rs-0287 */     split_words.sort_unstable();
/* FP:edit_distance.rs-0288 */     split_words
/* FP:edit_distance.rs-0289 */ }