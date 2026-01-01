/* FP:transitive_relation.rs-0001 */ use std::fmt::Debug;
/* FP:transitive_relation.rs-0002 */ use std::hash::Hash;
/* FP:transitive_relation.rs-0003 */ use std::mem;
/* FP:transitive_relation.rs-0004 */ use std::ops::Deref;
/* FP:transitive_relation.rs-0005 */ 
/* FP:transitive_relation.rs-0006 */ use crate::rustc_index::bit_set::BitMatrix;
/* FP:transitive_relation.rs-0007 */ 
/* FP:transitive_relation.rs-0008 */ use crate::frozen::Frozen;
/* FP:transitive_relation.rs-0009 */ use crate::fx::{FxHashSet, FxIndexSet};
/* FP:transitive_relation.rs-0010 */ 
/* FP:transitive_relation.rs-0011 */ #[cfg(test)]
/* FP:transitive_relation.rs-0013 */ 
/* FP:transitive_relation.rs-0014 */ #[derive(Clone, Debug)]
/* FP:transitive_relation.rs-0015 */ pub struct TransitiveRelationBuilder<T> {
/* FP:transitive_relation.rs-0016 */     // List of elements. This is used to map from a T to a usize.
/* FP:transitive_relation.rs-0017 */     elements: FxIndexSet<T>,
/* FP:transitive_relation.rs-0018 */ 
/* FP:transitive_relation.rs-0019 */     // List of base edges in the graph. Require to compute transitive
/* FP:transitive_relation.rs-0020 */     // closure.
/* FP:transitive_relation.rs-0021 */     edges: FxHashSet<Edge>,
/* FP:transitive_relation.rs-0022 */ }
/* FP:transitive_relation.rs-0023 */ 
/* FP:transitive_relation.rs-0024 */ #[derive(Debug)]
/* FP:transitive_relation.rs-0025 */ pub struct TransitiveRelation<T> {
/* FP:transitive_relation.rs-0026 */     // Frozen transitive relation elements and edges.
/* FP:transitive_relation.rs-0027 */     builder: Frozen<TransitiveRelationBuilder<T>>,
/* FP:transitive_relation.rs-0028 */ 
/* FP:transitive_relation.rs-0029 */     // Cached transitive closure derived from the edges.
/* FP:transitive_relation.rs-0030 */     closure: Frozen<BitMatrix<usize, usize>>,
/* FP:transitive_relation.rs-0031 */ }
/* FP:transitive_relation.rs-0032 */ 
/* FP:transitive_relation.rs-0033 */ impl<T> Deref for TransitiveRelation<T> {
/* FP:transitive_relation.rs-0034 */     type Target = Frozen<TransitiveRelationBuilder<T>>;
/* FP:transitive_relation.rs-0035 */ 
/* FP:transitive_relation.rs-0036 */     fn deref(&self) -> &Self::Target {
/* FP:transitive_relation.rs-0037 */         &self.builder
/* FP:transitive_relation.rs-0038 */     }
/* FP:transitive_relation.rs-0039 */ }
/* FP:transitive_relation.rs-0040 */ 
/* FP:transitive_relation.rs-0041 */ impl<T: Clone> Clone for TransitiveRelation<T> {
/* FP:transitive_relation.rs-0042 */     fn clone(&self) -> Self {
/* FP:transitive_relation.rs-0043 */         TransitiveRelation {
/* FP:transitive_relation.rs-0044 */             builder: Frozen::freeze(self.builder.deref().clone()),
/* FP:transitive_relation.rs-0045 */             closure: Frozen::freeze(self.closure.deref().clone()),
/* FP:transitive_relation.rs-0046 */         }
/* FP:transitive_relation.rs-0047 */     }
/* FP:transitive_relation.rs-0048 */ }
/* FP:transitive_relation.rs-0049 */ 
/* FP:transitive_relation.rs-0050 */ // HACK(eddyb) manual impl avoids `Default` bound on `T`.
/* FP:transitive_relation.rs-0051 */ impl<T: Eq + Hash> Default for TransitiveRelationBuilder<T> {
/* FP:transitive_relation.rs-0052 */     fn default() -> Self {
/* FP:transitive_relation.rs-0053 */         TransitiveRelationBuilder { elements: Default::default(), edges: Default::default() }
/* FP:transitive_relation.rs-0054 */     }
/* FP:transitive_relation.rs-0055 */ }
/* FP:transitive_relation.rs-0056 */ 
/* FP:transitive_relation.rs-0057 */ #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
/* FP:transitive_relation.rs-0058 */ struct Index(usize);
/* FP:transitive_relation.rs-0059 */ 
/* FP:transitive_relation.rs-0060 */ #[derive(Clone, PartialEq, Eq, Debug, Hash)]
/* FP:transitive_relation.rs-0061 */ struct Edge {
/* FP:transitive_relation.rs-0062 */     source: Index,
/* FP:transitive_relation.rs-0063 */     target: Index,
/* FP:transitive_relation.rs-0064 */ }
/* FP:transitive_relation.rs-0065 */ 
/* FP:transitive_relation.rs-0066 */ impl<T: Eq + Hash + Copy> TransitiveRelationBuilder<T> {
/* FP:transitive_relation.rs-0067 */     pub fn is_empty(&self) -> bool {
/* FP:transitive_relation.rs-0068 */         self.edges.is_empty()
/* FP:transitive_relation.rs-0069 */     }
/* FP:transitive_relation.rs-0070 */ 
/* FP:transitive_relation.rs-0071 */     pub fn elements(&self) -> impl Iterator<Item = &T> {
/* FP:transitive_relation.rs-0072 */         self.elements.iter()
/* FP:transitive_relation.rs-0073 */     }
/* FP:transitive_relation.rs-0074 */ 
/* FP:transitive_relation.rs-0075 */     fn index(&self, a: T) -> Option<Index> {
/* FP:transitive_relation.rs-0076 */         self.elements.get_index_of(&a).map(Index)
/* FP:transitive_relation.rs-0077 */     }
/* FP:transitive_relation.rs-0078 */ 
/* FP:transitive_relation.rs-0079 */     fn add_index(&mut self, a: T) -> Index {
/* FP:transitive_relation.rs-0080 */         let (index, _added) = self.elements.insert_full(a);
/* FP:transitive_relation.rs-0081 */         Index(index)
/* FP:transitive_relation.rs-0082 */     }
/* FP:transitive_relation.rs-0083 */ 
/* FP:transitive_relation.rs-0084 */     /// Applies the (partial) function to each edge and returns a new
/* FP:transitive_relation.rs-0085 */     /// relation builder. If `f` returns `None` for any end-point,
/* FP:transitive_relation.rs-0086 */     /// returns `None`.
/* FP:transitive_relation.rs-0087 */     pub fn maybe_map<F, U>(&self, mut f: F) -> Option<TransitiveRelationBuilder<U>>
/* FP:transitive_relation.rs-0088 */     where
/* FP:transitive_relation.rs-0089 */         F: FnMut(T) -> Option<U>,
/* FP:transitive_relation.rs-0090 */         U: Clone + Debug + Eq + Hash + Copy,
/* FP:transitive_relation.rs-0091 */     {
/* FP:transitive_relation.rs-0092 */         let mut result = TransitiveRelationBuilder::default();
/* FP:transitive_relation.rs-0093 */         for edge in &self.edges {
/* FP:transitive_relation.rs-0094 */             result.add(f(self.elements[edge.source.0])?, f(self.elements[edge.target.0])?);
/* FP:transitive_relation.rs-0095 */         }
/* FP:transitive_relation.rs-0096 */         Some(result)
/* FP:transitive_relation.rs-0097 */     }
/* FP:transitive_relation.rs-0098 */ 
/* FP:transitive_relation.rs-0099 */     /// Indicate that `a < b` (where `<` is this relation)
/* FP:transitive_relation.rs-0100 */     pub fn add(&mut self, a: T, b: T) {
/* FP:transitive_relation.rs-0101 */         let a = self.add_index(a);
/* FP:transitive_relation.rs-0102 */         let b = self.add_index(b);
/* FP:transitive_relation.rs-0103 */         let edge = Edge { source: a, target: b };
/* FP:transitive_relation.rs-0104 */         self.edges.insert(edge);
/* FP:transitive_relation.rs-0105 */     }
/* FP:transitive_relation.rs-0106 */ 
/* FP:transitive_relation.rs-0107 */     /// Compute the transitive closure derived from the edges, and converted to
/* FP:transitive_relation.rs-0108 */     /// the final result. After this, all elements will be immutable to maintain
/* FP:transitive_relation.rs-0109 */     /// the correctness of the result.
/* FP:transitive_relation.rs-0110 */     pub fn freeze(self) -> TransitiveRelation<T> {
/* FP:transitive_relation.rs-0111 */         let mut matrix = BitMatrix::new(self.elements.len(), self.elements.len());
/* FP:transitive_relation.rs-0112 */         let mut changed = true;
/* FP:transitive_relation.rs-0113 */         while changed {
/* FP:transitive_relation.rs-0114 */             changed = false;
/* FP:transitive_relation.rs-0115 */             for edge in &self.edges {
/* FP:transitive_relation.rs-0116 */                 // add an edge from S -> T
/* FP:transitive_relation.rs-0117 */                 changed |= matrix.insert(edge.source.0, edge.target.0);
/* FP:transitive_relation.rs-0118 */ 
/* FP:transitive_relation.rs-0119 */                 // add all outgoing edges from T into S
/* FP:transitive_relation.rs-0120 */                 changed |= matrix.union_rows(edge.target.0, edge.source.0);
/* FP:transitive_relation.rs-0121 */             }
/* FP:transitive_relation.rs-0122 */         }
/* FP:transitive_relation.rs-0123 */         TransitiveRelation { builder: Frozen::freeze(self), closure: Frozen::freeze(matrix) }
/* FP:transitive_relation.rs-0124 */     }
/* FP:transitive_relation.rs-0125 */ }
/* FP:transitive_relation.rs-0126 */ 
/* FP:transitive_relation.rs-0127 */ impl<T: Eq + Hash + Copy> TransitiveRelation<T> {
/* FP:transitive_relation.rs-0128 */     /// Applies the (partial) function to each edge and returns a new
/* FP:transitive_relation.rs-0129 */     /// relation including transitive closures.
/* FP:transitive_relation.rs-0130 */     pub fn maybe_map<F, U>(&self, f: F) -> Option<TransitiveRelation<U>>
/* FP:transitive_relation.rs-0131 */     where
/* FP:transitive_relation.rs-0132 */         F: FnMut(T) -> Option<U>,
/* FP:transitive_relation.rs-0133 */         U: Clone + Debug + Eq + Hash + Copy,
/* FP:transitive_relation.rs-0134 */     {
/* FP:transitive_relation.rs-0135 */         Some(self.builder.maybe_map(f)?.freeze())
/* FP:transitive_relation.rs-0136 */     }
/* FP:transitive_relation.rs-0137 */ 
/* FP:transitive_relation.rs-0138 */     /// Checks whether `a < target` (transitively)
/* FP:transitive_relation.rs-0139 */     pub fn contains(&self, a: T, b: T) -> bool {
/* FP:transitive_relation.rs-0140 */         match (self.index(a), self.index(b)) {
/* FP:transitive_relation.rs-0141 */             (Some(a), Some(b)) => self.with_closure(|closure| closure.contains(a.0, b.0)),
/* FP:transitive_relation.rs-0142 */             (None, _) | (_, None) => false,
/* FP:transitive_relation.rs-0143 */         }
/* FP:transitive_relation.rs-0144 */     }
/* FP:transitive_relation.rs-0145 */ 
/* FP:transitive_relation.rs-0146 */     /// Thinking of `x R y` as an edge `x -> y` in a graph, this
/* FP:transitive_relation.rs-0147 */     /// returns all things reachable from `a`.
/* FP:transitive_relation.rs-0148 */     ///
/* FP:transitive_relation.rs-0149 */     /// Really this probably ought to be `impl Iterator<Item = &T>`, but
/* FP:transitive_relation.rs-0150 */     /// I'm too lazy to make that work, and -- given the caching
/* FP:transitive_relation.rs-0151 */     /// strategy -- it'd be a touch tricky anyhow.
/* FP:transitive_relation.rs-0152 */     pub fn reachable_from(&self, a: T) -> Vec<T> {
/* FP:transitive_relation.rs-0153 */         match self.index(a) {
/* FP:transitive_relation.rs-0154 */             Some(a) => {
/* FP:transitive_relation.rs-0155 */                 self.with_closure(|closure| closure.iter(a.0).map(|i| self.elements[i]).collect())
/* FP:transitive_relation.rs-0156 */             }
/* FP:transitive_relation.rs-0157 */             None => vec![],
/* FP:transitive_relation.rs-0158 */         }
/* FP:transitive_relation.rs-0159 */     }
/* FP:transitive_relation.rs-0160 */ 
/* FP:transitive_relation.rs-0161 */     /// Picks what I am referring to as the "postdominating"
/* FP:transitive_relation.rs-0162 */     /// upper-bound for `a` and `b`. This is usually the least upper
/* FP:transitive_relation.rs-0163 */     /// bound, but in cases where there is no single least upper
/* FP:transitive_relation.rs-0164 */     /// bound, it is the "mutual immediate postdominator", if you
/* FP:transitive_relation.rs-0165 */     /// imagine a graph where `a < b` means `a -> b`.
/* FP:transitive_relation.rs-0166 */     ///
/* FP:transitive_relation.rs-0167 */     /// This function is needed because region inference currently
/* FP:transitive_relation.rs-0168 */     /// requires that we produce a single "UB", and there is no best
/* FP:transitive_relation.rs-0169 */     /// choice for the LUB. Rather than pick arbitrarily, I pick a
/* FP:transitive_relation.rs-0170 */     /// less good, but predictable choice. This should help ensure
/* FP:transitive_relation.rs-0171 */     /// that region inference yields predictable results (though it
/* FP:transitive_relation.rs-0172 */     /// itself is not fully sufficient).
/* FP:transitive_relation.rs-0173 */     ///
/* FP:transitive_relation.rs-0174 */     /// Examples are probably clearer than any prose I could write
/* FP:transitive_relation.rs-0175 */     /// (there are corresponding tests below, btw). In each case,
/* FP:transitive_relation.rs-0176 */     /// the query is `postdom_upper_bound(a, b)`:
/* FP:transitive_relation.rs-0177 */     ///
/* FP:transitive_relation.rs-0178 */     /// ```text
/* FP:transitive_relation.rs-0179 */     /// // Returns Some(x), which is also LUB.
/* FP:transitive_relation.rs-0180 */     /// a -> a1 -> x
/* FP:transitive_relation.rs-0181 */     ///            ^
/* FP:transitive_relation.rs-0182 */     ///            |
/* FP:transitive_relation.rs-0183 */     /// b -> b1 ---+
/* FP:transitive_relation.rs-0184 */     ///
/* FP:transitive_relation.rs-0185 */     /// // Returns `Some(x)`, which is not LUB (there is none)
/* FP:transitive_relation.rs-0186 */     /// // diagonal edges run left-to-right.
/* FP:transitive_relation.rs-0187 */     /// a -> a1 -> x
/* FP:transitive_relation.rs-0188 */     ///   \/       ^
/* FP:transitive_relation.rs-0189 */     ///   /\       |
/* FP:transitive_relation.rs-0190 */     /// b -> b1 ---+
/* FP:transitive_relation.rs-0191 */     ///
/* FP:transitive_relation.rs-0192 */     /// // Returns `None`.
/* FP:transitive_relation.rs-0193 */     /// a -> a1
/* FP:transitive_relation.rs-0194 */     /// b -> b1
/* FP:transitive_relation.rs-0195 */     /// ```
/* FP:transitive_relation.rs-0196 */     pub fn postdom_upper_bound(&self, a: T, b: T) -> Option<T> {
/* FP:transitive_relation.rs-0197 */         let mubs = self.minimal_upper_bounds(a, b);
/* FP:transitive_relation.rs-0198 */         self.mutual_immediate_postdominator(mubs)
/* FP:transitive_relation.rs-0199 */     }
/* FP:transitive_relation.rs-0200 */ 
/* FP:transitive_relation.rs-0201 */     /// Viewing the relation as a graph, computes the "mutual
/* FP:transitive_relation.rs-0202 */     /// immediate postdominator" of a set of points (if one
/* FP:transitive_relation.rs-0203 */     /// exists). See `postdom_upper_bound` for details.
/* FP:transitive_relation.rs-0204 */     pub fn mutual_immediate_postdominator(&self, mut mubs: Vec<T>) -> Option<T> {
/* FP:transitive_relation.rs-0205 */         loop {
/* FP:transitive_relation.rs-0206 */             match mubs[..] {
/* FP:transitive_relation.rs-0207 */                 [] => return None,
/* FP:transitive_relation.rs-0208 */                 [mub] => return Some(mub),
/* FP:transitive_relation.rs-0209 */                 _ => {
/* FP:transitive_relation.rs-0210 */                     let m = mubs.pop().unwrap();
/* FP:transitive_relation.rs-0211 */                     let n = mubs.pop().unwrap();
/* FP:transitive_relation.rs-0212 */                     mubs.extend(self.minimal_upper_bounds(n, m));
/* FP:transitive_relation.rs-0213 */                 }
/* FP:transitive_relation.rs-0214 */             }
/* FP:transitive_relation.rs-0215 */         }
/* FP:transitive_relation.rs-0216 */     }
/* FP:transitive_relation.rs-0217 */ 
/* FP:transitive_relation.rs-0218 */     /// Returns the set of bounds `X` such that:
/* FP:transitive_relation.rs-0219 */     ///
/* FP:transitive_relation.rs-0220 */     /// - `a < X` and `b < X`
/* FP:transitive_relation.rs-0221 */     /// - there is no `Y != X` such that `a < Y` and `Y < X`
/* FP:transitive_relation.rs-0222 */     ///   - except for the case where `X < a` (i.e., a strongly connected
/* FP:transitive_relation.rs-0223 */     ///     component in the graph). In that case, the smallest
/* FP:transitive_relation.rs-0224 */     ///     representative of the SCC is returned (as determined by the
/* FP:transitive_relation.rs-0225 */     ///     internal indices).
/* FP:transitive_relation.rs-0226 */     ///
/* FP:transitive_relation.rs-0227 */     /// Note that this set can, in principle, have any size.
/* FP:transitive_relation.rs-0228 */     pub fn minimal_upper_bounds(&self, a: T, b: T) -> Vec<T> {
/* FP:transitive_relation.rs-0229 */         let (Some(mut a), Some(mut b)) = (self.index(a), self.index(b)) else {
/* FP:transitive_relation.rs-0230 */             return vec![];
/* FP:transitive_relation.rs-0231 */         };
/* FP:transitive_relation.rs-0232 */ 
/* FP:transitive_relation.rs-0233 */         // in some cases, there are some arbitrary choices to be made;
/* FP:transitive_relation.rs-0234 */         // it doesn't really matter what we pick, as long as we pick
/* FP:transitive_relation.rs-0235 */         // the same thing consistently when queried, so ensure that
/* FP:transitive_relation.rs-0236 */         // (a, b) are in a consistent relative order
/* FP:transitive_relation.rs-0237 */         if a > b {
/* FP:transitive_relation.rs-0238 */             mem::swap(&mut a, &mut b);
/* FP:transitive_relation.rs-0239 */         }
/* FP:transitive_relation.rs-0240 */ 
/* FP:transitive_relation.rs-0241 */         let lub_indices = self.with_closure(|closure| {
/* FP:transitive_relation.rs-0242 */             // Easy case is when either a < b or b < a:
/* FP:transitive_relation.rs-0243 */             if closure.contains(a.0, b.0) {
/* FP:transitive_relation.rs-0244 */                 return vec![b.0];
/* FP:transitive_relation.rs-0245 */             }
/* FP:transitive_relation.rs-0246 */             if closure.contains(b.0, a.0) {
/* FP:transitive_relation.rs-0247 */                 return vec![a.0];
/* FP:transitive_relation.rs-0248 */             }
/* FP:transitive_relation.rs-0249 */ 
/* FP:transitive_relation.rs-0250 */             // Otherwise, the tricky part is that there may be some c
/* FP:transitive_relation.rs-0251 */             // where a < c and b < c. In fact, there may be many such
/* FP:transitive_relation.rs-0252 */             // values. So here is what we do:
/* FP:transitive_relation.rs-0253 */             //
/* FP:transitive_relation.rs-0254 */             // 1. Find the vector `[X | a < X && b < X]` of all values
/* FP:transitive_relation.rs-0255 */             //    `X` where `a < X` and `b < X`. In terms of the
/* FP:transitive_relation.rs-0256 */             //    graph, this means all values reachable from both `a`
/* FP:transitive_relation.rs-0257 */             //    and `b`. Note that this vector is also a set, but we
/* FP:transitive_relation.rs-0258 */             //    use the term vector because the order matters
/* FP:transitive_relation.rs-0259 */             //    to the steps below.
/* FP:transitive_relation.rs-0260 */             //    - This vector contains upper bounds, but they are
/* FP:transitive_relation.rs-0261 */             //      not minimal upper bounds. So you may have e.g.
/* FP:transitive_relation.rs-0262 */             //      `[x, y, tcx, z]` where `x < tcx` and `y < tcx` and
/* FP:transitive_relation.rs-0263 */             //      `z < x` and `z < y`:
/* FP:transitive_relation.rs-0264 */             //
/* FP:transitive_relation.rs-0265 */             //           z --+---> x ----+----> tcx
/* FP:transitive_relation.rs-0266 */             //               |           |
/* FP:transitive_relation.rs-0267 */             //               |           |
/* FP:transitive_relation.rs-0268 */             //               +---> y ----+
/* FP:transitive_relation.rs-0269 */             //
/* FP:transitive_relation.rs-0270 */             //      In this case, we really want to return just `[z]`.
/* FP:transitive_relation.rs-0271 */             //      The following steps below achieve this by gradually
/* FP:transitive_relation.rs-0272 */             //      reducing the list.
/* FP:transitive_relation.rs-0273 */             // 2. Pare down the vector using `pare_down`. This will
/* FP:transitive_relation.rs-0274 */             //    remove elements from the vector that can be reached
/* FP:transitive_relation.rs-0275 */             //    by an earlier element.
/* FP:transitive_relation.rs-0276 */             //    - In the example above, this would convert `[x, y,
/* FP:transitive_relation.rs-0277 */             //      tcx, z]` to `[x, y, z]`. Note that `x` and `y` are
/* FP:transitive_relation.rs-0278 */             //      still in the vector; this is because while `z < x`
/* FP:transitive_relation.rs-0279 */             //      (and `z < y`) holds, `z` comes after them in the
/* FP:transitive_relation.rs-0280 */             //      vector.
/* FP:transitive_relation.rs-0281 */             // 3. Reverse the vector and repeat the pare down process.
/* FP:transitive_relation.rs-0282 */             //    - In the example above, we would reverse to
/* FP:transitive_relation.rs-0283 */             //      `[z, y, x]` and then pare down to `[z]`.
/* FP:transitive_relation.rs-0284 */             // 4. Reverse once more just so that we yield a vector in
/* FP:transitive_relation.rs-0285 */             //    increasing order of index. Not necessary, but why not.
/* FP:transitive_relation.rs-0286 */             //
/* FP:transitive_relation.rs-0287 */             // I believe this algorithm yields a minimal set. The
/* FP:transitive_relation.rs-0288 */             // argument is that, after step 2, we know that no element
/* FP:transitive_relation.rs-0289 */             // can reach its successors (in the vector, not the graph).
/* FP:transitive_relation.rs-0290 */             // After step 3, we know that no element can reach any of
/* FP:transitive_relation.rs-0291 */             // its predecessors (because of step 2) nor successors
/* FP:transitive_relation.rs-0292 */             // (because we just called `pare_down`)
/* FP:transitive_relation.rs-0293 */             //
/* FP:transitive_relation.rs-0294 */             // This same algorithm is used in `parents` below.
/* FP:transitive_relation.rs-0295 */ 
/* FP:transitive_relation.rs-0296 */             let mut candidates = closure.intersect_rows(a.0, b.0); // (1)
/* FP:transitive_relation.rs-0297 */             pare_down(&mut candidates, closure); // (2)
/* FP:transitive_relation.rs-0298 */             candidates.reverse(); // (3a)
/* FP:transitive_relation.rs-0299 */             pare_down(&mut candidates, closure); // (3b)
/* FP:transitive_relation.rs-0300 */             candidates
/* FP:transitive_relation.rs-0301 */         });
/* FP:transitive_relation.rs-0302 */ 
/* FP:transitive_relation.rs-0303 */         lub_indices
/* FP:transitive_relation.rs-0304 */             .into_iter()
/* FP:transitive_relation.rs-0305 */             .rev() // (4)
/* FP:transitive_relation.rs-0306 */             .map(|i| self.elements[i])
/* FP:transitive_relation.rs-0307 */             .collect()
/* FP:transitive_relation.rs-0308 */     }
/* FP:transitive_relation.rs-0309 */ 
/* FP:transitive_relation.rs-0310 */     /// Given an element A, returns the maximal set {B} of elements B
/* FP:transitive_relation.rs-0311 */     /// such that
/* FP:transitive_relation.rs-0312 */     ///
/* FP:transitive_relation.rs-0313 */     /// - A != B
/* FP:transitive_relation.rs-0314 */     /// - A R B is true
/* FP:transitive_relation.rs-0315 */     /// - for each i, j: `B[i]` R `B[j]` does not hold
/* FP:transitive_relation.rs-0316 */     ///
/* FP:transitive_relation.rs-0317 */     /// The intuition is that this moves "one step up" through a lattice
/* FP:transitive_relation.rs-0318 */     /// (where the relation is encoding the `<=` relation for the lattice).
/* FP:transitive_relation.rs-0319 */     /// So e.g., if the relation is `->` and we have
/* FP:transitive_relation.rs-0320 */     ///
/* FP:transitive_relation.rs-0321 */     /// ```text
/* FP:transitive_relation.rs-0322 */     /// a -> b -> d -> f
/* FP:transitive_relation.rs-0323 */     /// |              ^
/* FP:transitive_relation.rs-0324 */     /// +--> c -> e ---+
/* FP:transitive_relation.rs-0325 */     /// ```
/* FP:transitive_relation.rs-0326 */     ///
/* FP:transitive_relation.rs-0327 */     /// then `parents(a)` returns `[b, c]`. The `postdom_parent` function
/* FP:transitive_relation.rs-0328 */     /// would further reduce this to just `f`.
/* FP:transitive_relation.rs-0329 */     pub fn parents(&self, a: T) -> Vec<T> {
/* FP:transitive_relation.rs-0330 */         let Some(a) = self.index(a) else {
/* FP:transitive_relation.rs-0331 */             return vec![];
/* FP:transitive_relation.rs-0332 */         };
/* FP:transitive_relation.rs-0333 */ 
/* FP:transitive_relation.rs-0334 */         // Steal the algorithm for `minimal_upper_bounds` above, but
/* FP:transitive_relation.rs-0335 */         // with a slight tweak. In the case where `a R a`, we remove
/* FP:transitive_relation.rs-0336 */         // that from the set of candidates.
/* FP:transitive_relation.rs-0337 */         let ancestors = self.with_closure(|closure| {
/* FP:transitive_relation.rs-0338 */             let mut ancestors = closure.intersect_rows(a.0, a.0);
/* FP:transitive_relation.rs-0339 */ 
/* FP:transitive_relation.rs-0340 */             // Remove anything that can reach `a`. If this is a
/* FP:transitive_relation.rs-0341 */             // reflexive relation, this will include `a` itself.
/* FP:transitive_relation.rs-0342 */             ancestors.retain(|&e| !closure.contains(e, a.0));
/* FP:transitive_relation.rs-0343 */ 
/* FP:transitive_relation.rs-0344 */             pare_down(&mut ancestors, closure); // (2)
/* FP:transitive_relation.rs-0345 */             ancestors.reverse(); // (3a)
/* FP:transitive_relation.rs-0346 */             pare_down(&mut ancestors, closure); // (3b)
/* FP:transitive_relation.rs-0347 */             ancestors
/* FP:transitive_relation.rs-0348 */         });
/* FP:transitive_relation.rs-0349 */ 
/* FP:transitive_relation.rs-0350 */         ancestors
/* FP:transitive_relation.rs-0351 */             .into_iter()
/* FP:transitive_relation.rs-0352 */             .rev() // (4)
/* FP:transitive_relation.rs-0353 */             .map(|i| self.elements[i])
/* FP:transitive_relation.rs-0354 */             .collect()
/* FP:transitive_relation.rs-0355 */     }
/* FP:transitive_relation.rs-0356 */ 
/* FP:transitive_relation.rs-0357 */     /// Given an element A, elements B with the lowest index such that `A R B`
/* FP:transitive_relation.rs-0358 */     /// and `B R A`, or `A` if no such element exists.
/* FP:transitive_relation.rs-0359 */     pub fn minimal_scc_representative(&self, a: T) -> T {
/* FP:transitive_relation.rs-0360 */         match self.index(a) {
/* FP:transitive_relation.rs-0361 */             Some(a_i) => self.with_closure(|closure| {
/* FP:transitive_relation.rs-0362 */                 closure
/* FP:transitive_relation.rs-0363 */                     .iter(a_i.0)
/* FP:transitive_relation.rs-0364 */                     .find(|i| closure.contains(*i, a_i.0))
/* FP:transitive_relation.rs-0365 */                     .map_or(a, |i| self.elements[i])
/* FP:transitive_relation.rs-0366 */             }),
/* FP:transitive_relation.rs-0367 */             None => a,
/* FP:transitive_relation.rs-0368 */         }
/* FP:transitive_relation.rs-0369 */     }
/* FP:transitive_relation.rs-0370 */ 
/* FP:transitive_relation.rs-0371 */     fn with_closure<OP, R>(&self, op: OP) -> R
/* FP:transitive_relation.rs-0372 */     where
/* FP:transitive_relation.rs-0373 */         OP: FnOnce(&BitMatrix<usize, usize>) -> R,
/* FP:transitive_relation.rs-0374 */     {
/* FP:transitive_relation.rs-0375 */         op(&self.closure)
/* FP:transitive_relation.rs-0376 */     }
/* FP:transitive_relation.rs-0377 */ 
/* FP:transitive_relation.rs-0378 */     /// Lists all the base edges in the graph: the initial _non-transitive_ set of element
/* FP:transitive_relation.rs-0379 */     /// relations, which will be later used as the basis for the transitive closure computation.
/* FP:transitive_relation.rs-0380 */     pub fn base_edges(&self) -> impl Iterator<Item = (T, T)> {
/* FP:transitive_relation.rs-0381 */         self.edges
/* FP:transitive_relation.rs-0382 */             .iter()
/* FP:transitive_relation.rs-0383 */             .map(move |edge| (self.elements[edge.source.0], self.elements[edge.target.0]))
/* FP:transitive_relation.rs-0384 */     }
/* FP:transitive_relation.rs-0385 */ }
/* FP:transitive_relation.rs-0386 */ 
/* FP:transitive_relation.rs-0387 */ /// Pare down is used as a step in the LUB computation. It edits the
/* FP:transitive_relation.rs-0388 */ /// candidates array in place by removing any element j for which
/* FP:transitive_relation.rs-0389 */ /// there exists an earlier element i<j such that i -> j. That is,
/* FP:transitive_relation.rs-0390 */ /// after you run `pare_down`, you know that for all elements that
/* FP:transitive_relation.rs-0391 */ /// remain in candidates, they cannot reach any of the elements that
/* FP:transitive_relation.rs-0392 */ /// come after them.
/* FP:transitive_relation.rs-0393 */ ///
/* FP:transitive_relation.rs-0394 */ /// Examples follow. Assume that a -> b -> c and x -> y -> z.
/* FP:transitive_relation.rs-0395 */ ///
/* FP:transitive_relation.rs-0396 */ /// - Input: `[a, b, x]`. Output: `[a, x]`.
/* FP:transitive_relation.rs-0397 */ /// - Input: `[b, a, x]`. Output: `[b, a, x]`.
/* FP:transitive_relation.rs-0398 */ /// - Input: `[a, x, b, y]`. Output: `[a, x]`.
/* FP:transitive_relation.rs-0399 */ fn pare_down(candidates: &mut Vec<usize>, closure: &BitMatrix<usize, usize>) {
/* FP:transitive_relation.rs-0400 */     let mut i = 0;
/* FP:transitive_relation.rs-0401 */     while let Some(&candidate_i) = candidates.get(i) {
/* FP:transitive_relation.rs-0402 */         i += 1;
/* FP:transitive_relation.rs-0403 */ 
/* FP:transitive_relation.rs-0404 */         let mut j = i;
/* FP:transitive_relation.rs-0405 */         let mut dead = 0;
/* FP:transitive_relation.rs-0406 */         while let Some(&candidate_j) = candidates.get(j) {
/* FP:transitive_relation.rs-0407 */             if closure.contains(candidate_i, candidate_j) {
/* FP:transitive_relation.rs-0408 */                 // If `i` can reach `j`, then we can remove `j`. So just
/* FP:transitive_relation.rs-0409 */                 // mark it as dead and move on; subsequent indices will be
/* FP:transitive_relation.rs-0410 */                 // shifted into its place.
/* FP:transitive_relation.rs-0411 */                 dead += 1;
/* FP:transitive_relation.rs-0412 */             } else {
/* FP:transitive_relation.rs-0413 */                 candidates[j - dead] = candidate_j;
/* FP:transitive_relation.rs-0414 */             }
/* FP:transitive_relation.rs-0415 */             j += 1;
/* FP:transitive_relation.rs-0416 */         }
/* FP:transitive_relation.rs-0417 */         candidates.truncate(j - dead);
/* FP:transitive_relation.rs-0418 */     }
/* FP:transitive_relation.rs-0419 */ }