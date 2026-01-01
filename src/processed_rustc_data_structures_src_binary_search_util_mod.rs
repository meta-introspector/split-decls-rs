/* FP:mod.rs-0001 */ #[cfg(test)]
/* FP:mod.rs-0003 */ 
/* FP:mod.rs-0004 */ /// Uses a sorted slice `data: &[E]` as a kind of "multi-map". The
/* FP:mod.rs-0005 */ /// `key_fn` extracts a key of type `K` from the data, and this
/* FP:mod.rs-0006 */ /// function finds the range of elements that match the key. `data`
/* FP:mod.rs-0007 */ /// must have been sorted as if by a call to `sort_by_key` for this to
/* FP:mod.rs-0008 */ /// work.
/* FP:mod.rs-0009 */ pub fn binary_search_slice<'d, E, K>(data: &'d [E], key_fn: impl Fn(&E) -> K, key: &K) -> &'d [E]
/* FP:mod.rs-0010 */ where
/* FP:mod.rs-0011 */     K: Ord,
/* FP:mod.rs-0012 */ {
/* FP:mod.rs-0013 */     let size = data.len();
/* FP:mod.rs-0014 */     let start = data.partition_point(|x| key_fn(x) < *key);
/* FP:mod.rs-0015 */     // At this point `start` either points at the first entry with equal or
/* FP:mod.rs-0016 */     // greater key or is equal to `size` in case all elements have smaller keys
/* FP:mod.rs-0017 */     if start == size || key_fn(&data[start]) != *key {
/* FP:mod.rs-0018 */         return &[];
/* FP:mod.rs-0019 */     };
/* FP:mod.rs-0020 */ 
/* FP:mod.rs-0021 */     // Find the first entry with key > `key`. Skip `start` entries since
/* FP:mod.rs-0022 */     // key_fn(&data[start]) == *key
/* FP:mod.rs-0023 */     let offset = start + 1;
/* FP:mod.rs-0024 */     let end = data[offset..].partition_point(|x| key_fn(x) <= *key) + offset;
/* FP:mod.rs-0025 */ 
/* FP:mod.rs-0026 */     &data[start..end]
/* FP:mod.rs-0027 */ }