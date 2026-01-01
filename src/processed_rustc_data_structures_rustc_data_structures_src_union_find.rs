/* FP:union_find.rs-0001 */ use std::cmp::Ordering;
/* FP:union_find.rs-0002 */ use std::mem;
/* FP:union_find.rs-0003 */ 
/* FP:union_find.rs-0004 */ use crate::rustc_index::{Idx, IndexVec};
/* FP:union_find.rs-0005 */ 
/* FP:union_find.rs-0006 */ #[cfg(test)]
/* FP:union_find.rs-0008 */ 
/* FP:union_find.rs-0009 */ /// Simple implementation of a union-find data structure, i.e. a disjoint-set
/* FP:union_find.rs-0010 */ /// forest.
/* FP:union_find.rs-0011 */ #[derive(Debug)]
/* FP:union_find.rs-0012 */ pub struct UnionFind<Key: Idx> {
/* FP:union_find.rs-0013 */     table: IndexVec<Key, UnionFindEntry<Key>>,
/* FP:union_find.rs-0014 */ }
/* FP:union_find.rs-0015 */ 
/* FP:union_find.rs-0016 */ #[derive(Debug)]
/* FP:union_find.rs-0017 */ struct UnionFindEntry<Key> {
/* FP:union_find.rs-0018 */     /// Transitively points towards the "root" of the set containing this key.
/* FP:union_find.rs-0019 */     ///
/* FP:union_find.rs-0020 */     /// Invariant: A root key is its own parent.
/* FP:union_find.rs-0021 */     parent: Key,
/* FP:union_find.rs-0022 */     /// When merging two "root" keys, their ranks determine which key becomes
/* FP:union_find.rs-0023 */     /// the new root, to prevent the parent tree from becoming unnecessarily
/* FP:union_find.rs-0024 */     /// tall. See [`UnionFind::unify`] for details.
/* FP:union_find.rs-0025 */     rank: u32,
/* FP:union_find.rs-0026 */ }
/* FP:union_find.rs-0027 */ 
/* FP:union_find.rs-0028 */ impl<Key: Idx> UnionFind<Key> {
/* FP:union_find.rs-0029 */     /// Creates a new disjoint-set forest containing the keys `0..num_keys`.
/* FP:union_find.rs-0030 */     /// Initially, every key is part of its own one-element set.
/* FP:union_find.rs-0031 */     pub fn new(num_keys: usize) -> Self {
/* FP:union_find.rs-0032 */         // Initially, every key is the root of its own set, so its parent is itself.
/* FP:union_find.rs-0033 */         Self { table: IndexVec::from_fn_n(|key| UnionFindEntry { parent: key, rank: 0 }, num_keys) }
/* FP:union_find.rs-0034 */     }
/* FP:union_find.rs-0035 */ 
/* FP:union_find.rs-0036 */     /// Returns the "root" key of the disjoint-set containing the given key.
/* FP:union_find.rs-0037 */     /// If two keys have the same root, they belong to the same set.
/* FP:union_find.rs-0038 */     ///
/* FP:union_find.rs-0039 */     /// Also updates internal data structures to make subsequent `find`
/* FP:union_find.rs-0040 */     /// operations faster.
/* FP:union_find.rs-0041 */     pub fn find(&mut self, key: Key) -> Key {
/* FP:union_find.rs-0042 */         // Loop until we find a key that is its own parent.
/* FP:union_find.rs-0043 */         let mut curr = key;
/* FP:union_find.rs-0044 */         while let parent = self.table[curr].parent
/* FP:union_find.rs-0045 */             && curr != parent
/* FP:union_find.rs-0046 */         {
/* FP:union_find.rs-0047 */             // Perform "path compression" by peeking one layer ahead, and
/* FP:union_find.rs-0048 */             // setting the current key's parent to that value.
/* FP:union_find.rs-0049 */             // (This works even when `parent` is the root of its set, because
/* FP:union_find.rs-0050 */             // of the invariant that a root is its own parent.)
/* FP:union_find.rs-0051 */             let parent_parent = self.table[parent].parent;
/* FP:union_find.rs-0052 */             self.table[curr].parent = parent_parent;
/* FP:union_find.rs-0053 */ 
/* FP:union_find.rs-0054 */             // Advance by one step and continue.
/* FP:union_find.rs-0055 */             curr = parent;
/* FP:union_find.rs-0056 */         }
/* FP:union_find.rs-0057 */         curr
/* FP:union_find.rs-0058 */     }
/* FP:union_find.rs-0059 */ 
/* FP:union_find.rs-0060 */     /// Merges the set containing `a` and the set containing `b` into one set.
/* FP:union_find.rs-0061 */     ///
/* FP:union_find.rs-0062 */     /// Returns the common root of both keys, after the merge.
/* FP:union_find.rs-0063 */     pub fn unify(&mut self, a: Key, b: Key) -> Key {
/* FP:union_find.rs-0064 */         let mut a = self.find(a);
/* FP:union_find.rs-0065 */         let mut b = self.find(b);
/* FP:union_find.rs-0066 */ 
/* FP:union_find.rs-0067 */         // If both keys have the same root, they're already in the same set,
/* FP:union_find.rs-0068 */         // so there's nothing more to do.
/* FP:union_find.rs-0069 */         if a == b {
/* FP:union_find.rs-0070 */             return a;
/* FP:union_find.rs-0071 */         };
/* FP:union_find.rs-0072 */ 
/* FP:union_find.rs-0073 */         // Ensure that `a` has strictly greater rank, swapping if necessary.
/* FP:union_find.rs-0074 */         // If both keys have the same rank, increment the rank of `a` so that
/* FP:union_find.rs-0075 */         // future unifications will also prefer `a`, leading to flatter trees.
/* FP:union_find.rs-0076 */         match Ord::cmp(&self.table[a].rank, &self.table[b].rank) {
/* FP:union_find.rs-0077 */             Ordering::Less => mem::swap(&mut a, &mut b),
/* FP:union_find.rs-0078 */             Ordering::Equal => self.table[a].rank += 1,
/* FP:union_find.rs-0079 */             Ordering::Greater => {}
/* FP:union_find.rs-0080 */         }
/* FP:union_find.rs-0081 */ 
/* FP:union_find.rs-0082 */         debug_assert!(self.table[a].rank > self.table[b].rank);
/* FP:union_find.rs-0083 */         debug_assert_eq!(self.table[b].parent, b);
/* FP:union_find.rs-0084 */ 
/* FP:union_find.rs-0085 */         // Make `a` the parent of `b`.
/* FP:union_find.rs-0086 */         self.table[b].parent = a;
/* FP:union_find.rs-0087 */ 
/* FP:union_find.rs-0088 */         a
/* FP:union_find.rs-0089 */     }
/* FP:union_find.rs-0090 */ 
/* FP:union_find.rs-0091 */     /// Takes a "snapshot" of the current state of this disjoint-set forest, in
/* FP:union_find.rs-0092 */     /// the form of a vector that directly maps each key to its current root.
/* FP:union_find.rs-0093 */     pub fn snapshot(&mut self) -> IndexVec<Key, Key> {
/* FP:union_find.rs-0094 */         self.table.indices().map(|key| self.find(key)).collect()
/* FP:union_find.rs-0095 */     }
/* FP:union_find.rs-0096 */ }