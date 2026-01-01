/* FP:mod.rs-0001 */ // Routine to compute the strongly connected components (SCCs) of a graph.
/* FP:mod.rs-0002 */ //
/* FP:mod.rs-0003 */ // Also computes as the resulting DAG if each SCC is replaced with a
/* FP:mod.rs-0004 */ // node in the graph. This uses [Tarjan's algorithm](
/* FP:mod.rs-0005 */ // https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm)
/* FP:mod.rs-0006 */ // that completes in *O*(*n*) time.
/* FP:mod.rs-0007 */ // Optionally, also annotate the SCC nodes with some commutative data.
/* FP:mod.rs-0008 */ // Typical examples would include: minimum element in SCC, maximum element
/* FP:mod.rs-0009 */ // reachable from it, etc.
/* FP:mod.rs-0010 */ 
/* FP:mod.rs-0011 */ use std::assert_matches::debug_assert_matches;
/* FP:mod.rs-0012 */ use std::fmt::Debug;
/* FP:mod.rs-0013 */ use std::marker::PhantomData;
/* FP:mod.rs-0014 */ use std::ops::Range;
/* FP:mod.rs-0015 */ 
/* FP:mod.rs-0016 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:mod.rs-0017 */ use tracing::{debug, instrument, trace};
/* FP:mod.rs-0018 */ 
/* FP:mod.rs-0019 */ use crate::fx::FxHashSet;
/* FP:mod.rs-0020 */ use crate::graph::vec_graph::VecGraph;
/* FP:mod.rs-0021 */ use crate::graph::{DirectedGraph, NumEdges, Successors};
/* FP:mod.rs-0022 */ 
/* FP:mod.rs-0023 */ #[cfg(test)]
/* FP:mod.rs-0025 */ 
/* FP:mod.rs-0026 */ /// An annotation for an SCC. This can be a representative,
/* FP:mod.rs-0027 */ /// the max/min element of the SCC, or all of the above.
/* FP:mod.rs-0028 */ ///
/* FP:mod.rs-0029 */ /// Concretely, the both merge operations must commute, e.g. where `merge`
/* FP:mod.rs-0030 */ /// is `merge_scc` and `merge_reached`: `a.merge(b) == b.merge(a)`
/* FP:mod.rs-0031 */ ///
/* FP:mod.rs-0032 */ /// In general, what you want is probably always min/max according
/* FP:mod.rs-0033 */ /// to some ordering, potentially with side constraints (min x such
/* FP:mod.rs-0034 */ /// that P holds).
/* FP:mod.rs-0035 */ pub trait Annotation: Debug + Copy {
/* FP:mod.rs-0036 */     /// Merge two existing annotations into one during
/* FP:mod.rs-0037 */     /// path compression.o
/* FP:mod.rs-0038 */     fn merge_scc(self, other: Self) -> Self;
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */     /// Merge a successor into this annotation.
/* FP:mod.rs-0041 */     fn merge_reached(self, other: Self) -> Self;
/* FP:mod.rs-0042 */ 
/* FP:mod.rs-0043 */     fn update_scc(&mut self, other: Self) {
/* FP:mod.rs-0044 */         *self = self.merge_scc(other)
/* FP:mod.rs-0045 */     }
/* FP:mod.rs-0046 */ 
/* FP:mod.rs-0047 */     fn update_reachable(&mut self, other: Self) {
/* FP:mod.rs-0048 */         *self = self.merge_reached(other)
/* FP:mod.rs-0049 */     }
/* FP:mod.rs-0050 */ }
/* FP:mod.rs-0051 */ 
/* FP:mod.rs-0052 */ /// An accumulator for annotations.
/* FP:mod.rs-0053 */ pub trait Annotations<N: Idx> {
/* FP:mod.rs-0054 */     type Ann: Annotation;
/* FP:mod.rs-0055 */     type SccIdx: Idx + Ord;
/* FP:mod.rs-0056 */ 
/* FP:mod.rs-0057 */     fn new(&self, element: N) -> Self::Ann;
/* FP:mod.rs-0058 */     fn annotate_scc(&mut self, scc: Self::SccIdx, annotation: Self::Ann);
/* FP:mod.rs-0059 */ }
/* FP:mod.rs-0060 */ 
/* FP:mod.rs-0061 */ /// The nil annotation accumulator, which does nothing.
/* FP:mod.rs-0062 */ struct NoAnnotations<S: Idx + Ord>(PhantomData<S>);
/* FP:mod.rs-0063 */ 
/* FP:mod.rs-0064 */ impl<N: Idx, S: Idx + Ord> Annotations<N> for NoAnnotations<S> {
/* FP:mod.rs-0065 */     type SccIdx = S;
/* FP:mod.rs-0066 */     type Ann = ();
/* FP:mod.rs-0067 */     fn new(&self, _element: N) {}
/* FP:mod.rs-0068 */     fn annotate_scc(&mut self, _scc: S, _annotation: ()) {}
/* FP:mod.rs-0069 */ }
/* FP:mod.rs-0070 */ 
/* FP:mod.rs-0071 */ /// The empty annotation, which does nothing.
/* FP:mod.rs-0072 */ impl Annotation for () {
/* FP:mod.rs-0073 */     fn merge_reached(self, _other: Self) -> Self {
/* FP:mod.rs-0074 */         ()
/* FP:mod.rs-0075 */     }
/* FP:mod.rs-0076 */     fn merge_scc(self, _other: Self) -> Self {
/* FP:mod.rs-0077 */         ()
/* FP:mod.rs-0078 */     }
/* FP:mod.rs-0079 */ }
/* FP:mod.rs-0080 */ 
/* FP:mod.rs-0081 */ /// Strongly connected components (SCC) of a graph. The type `N` is
/* FP:mod.rs-0082 */ /// the index type for the graph nodes and `S` is the index type for
/* FP:mod.rs-0083 */ /// the SCCs. We can map from each node to the SCC that it
/* FP:mod.rs-0084 */ /// participates in, and we also have the successors of each SCC.
/* FP:mod.rs-0085 */ pub struct Sccs<N: Idx, S: Idx> {
/* FP:mod.rs-0086 */     /// For each node, what is the SCC index of the SCC to which it
/* FP:mod.rs-0087 */     /// belongs.
/* FP:mod.rs-0088 */     scc_indices: IndexVec<N, S>,
/* FP:mod.rs-0089 */ 
/* FP:mod.rs-0090 */     /// Data about all the SCCs.
/* FP:mod.rs-0091 */     scc_data: SccData<S>,
/* FP:mod.rs-0092 */ }
/* FP:mod.rs-0093 */ 
/* FP:mod.rs-0094 */ /// Information about an invidividual SCC node.
/* FP:mod.rs-0095 */ struct SccDetails {
/* FP:mod.rs-0096 */     /// For this SCC, the range of `all_successors` where its
/* FP:mod.rs-0097 */     /// successors can be found.
/* FP:mod.rs-0098 */     range: Range<usize>,
/* FP:mod.rs-0099 */ }
/* FP:mod.rs-0100 */ 
/* FP:mod.rs-0101 */ // The name of this struct should discourage you from making it public and leaking
/* FP:mod.rs-0102 */ // its representation. This message was left here by one who came before you,
/* FP:mod.rs-0103 */ // who learnt the hard way that making even small changes in representation
/* FP:mod.rs-0104 */ // is difficult when it's publicly inspectable.
/* FP:mod.rs-0105 */ //
/* FP:mod.rs-0106 */ // Obey the law of Demeter!
/* FP:mod.rs-0107 */ struct SccData<S: Idx> {
/* FP:mod.rs-0108 */     /// Maps SCC indices to their metadata, including
/* FP:mod.rs-0109 */     /// offsets into `all_successors`.
/* FP:mod.rs-0110 */     scc_details: IndexVec<S, SccDetails>,
/* FP:mod.rs-0111 */ 
/* FP:mod.rs-0112 */     /// Contains the successors for all the Sccs, concatenated. The
/* FP:mod.rs-0113 */     /// range of indices corresponding to a given SCC is found in its
/* FP:mod.rs-0114 */     /// `scc_details.range`.
/* FP:mod.rs-0115 */     all_successors: Vec<S>,
/* FP:mod.rs-0116 */ }
/* FP:mod.rs-0117 */ 
/* FP:mod.rs-0118 */ impl<N: Idx, S: Idx + Ord> Sccs<N, S> {
/* FP:mod.rs-0119 */     /// Compute SCCs without annotations.
/* FP:mod.rs-0120 */     pub fn new(graph: &impl Successors<Node = N>) -> Self {
/* FP:mod.rs-0121 */         Self::new_with_annotation(graph, &mut NoAnnotations(PhantomData::<S>))
/* FP:mod.rs-0122 */     }
/* FP:mod.rs-0123 */ 
/* FP:mod.rs-0124 */     /// Compute SCCs and annotate them with a user-supplied annotation
/* FP:mod.rs-0125 */     pub fn new_with_annotation<A: Annotations<N, SccIdx = S>>(
/* FP:mod.rs-0126 */         graph: &impl Successors<Node = N>,
/* FP:mod.rs-0127 */         annotations: &mut A,
/* FP:mod.rs-0128 */     ) -> Self {
/* FP:mod.rs-0129 */         SccsConstruction::construct(graph, annotations)
/* FP:mod.rs-0130 */     }
/* FP:mod.rs-0131 */ 
/* FP:mod.rs-0132 */     pub fn scc_indices(&self) -> &IndexSlice<N, S> {
/* FP:mod.rs-0133 */         &self.scc_indices
/* FP:mod.rs-0134 */     }
/* FP:mod.rs-0135 */ 
/* FP:mod.rs-0136 */     /// Returns the number of SCCs in the graph.
/* FP:mod.rs-0137 */     pub fn num_sccs(&self) -> usize {
/* FP:mod.rs-0138 */         self.scc_data.len()
/* FP:mod.rs-0139 */     }
/* FP:mod.rs-0140 */ 
/* FP:mod.rs-0141 */     /// Returns an iterator over the SCCs in the graph.
/* FP:mod.rs-0142 */     ///
/* FP:mod.rs-0143 */     /// The SCCs will be iterated in **dependency order** (or **post order**),
/* FP:mod.rs-0144 */     /// meaning that if `S1 -> S2`, we will visit `S2` first and `S1` after.
/* FP:mod.rs-0145 */     /// This is convenient when the edges represent dependencies: when you visit
/* FP:mod.rs-0146 */     /// `S1`, the value for `S2` will already have been computed.
/* FP:mod.rs-0147 */     pub fn all_sccs(&self) -> impl Iterator<Item = S> + 'static {
/* FP:mod.rs-0148 */         (0..self.scc_data.len()).map(S::new)
/* FP:mod.rs-0149 */     }
/* FP:mod.rs-0150 */ 
/* FP:mod.rs-0151 */     /// Returns the SCC to which a node `r` belongs.
/* FP:mod.rs-0152 */     pub fn scc(&self, r: N) -> S {
/* FP:mod.rs-0153 */         self.scc_indices[r]
/* FP:mod.rs-0154 */     }
/* FP:mod.rs-0155 */ 
/* FP:mod.rs-0156 */     /// Returns the successors of the given SCC.
/* FP:mod.rs-0157 */     pub fn successors(&self, scc: S) -> &[S] {
/* FP:mod.rs-0158 */         self.scc_data.successors(scc)
/* FP:mod.rs-0159 */     }
/* FP:mod.rs-0160 */ 
/* FP:mod.rs-0161 */     /// Construct the reverse graph of the SCC graph.
/* FP:mod.rs-0162 */     pub fn reverse(&self) -> VecGraph<S> {
/* FP:mod.rs-0163 */         VecGraph::new(
/* FP:mod.rs-0164 */             self.num_sccs(),
/* FP:mod.rs-0165 */             self.all_sccs()
/* FP:mod.rs-0166 */                 .flat_map(|source| {
/* FP:mod.rs-0167 */                     self.successors(source).iter().map(move |&target| (target, source))
/* FP:mod.rs-0168 */                 })
/* FP:mod.rs-0169 */                 .collect(),
/* FP:mod.rs-0170 */         )
/* FP:mod.rs-0171 */     }
/* FP:mod.rs-0172 */ }
/* FP:mod.rs-0173 */ 
/* FP:mod.rs-0174 */ impl<N: Idx, S: Idx + Ord> DirectedGraph for Sccs<N, S> {
/* FP:mod.rs-0175 */     type Node = S;
/* FP:mod.rs-0176 */ 
/* FP:mod.rs-0177 */     fn num_nodes(&self) -> usize {
/* FP:mod.rs-0178 */         self.num_sccs()
/* FP:mod.rs-0179 */     }
/* FP:mod.rs-0180 */ }
/* FP:mod.rs-0181 */ 
/* FP:mod.rs-0182 */ impl<N: Idx, S: Idx + Ord> NumEdges for Sccs<N, S> {
/* FP:mod.rs-0183 */     fn num_edges(&self) -> usize {
/* FP:mod.rs-0184 */         self.scc_data.all_successors.len()
/* FP:mod.rs-0185 */     }
/* FP:mod.rs-0186 */ }
/* FP:mod.rs-0187 */ 
/* FP:mod.rs-0188 */ impl<N: Idx, S: Idx + Ord> Successors for Sccs<N, S> {
/* FP:mod.rs-0189 */     fn successors(&self, node: S) -> impl Iterator<Item = Self::Node> {
/* FP:mod.rs-0190 */         self.successors(node).iter().cloned()
/* FP:mod.rs-0191 */     }
/* FP:mod.rs-0192 */ }
/* FP:mod.rs-0193 */ 
/* FP:mod.rs-0194 */ impl<S: Idx> SccData<S> {
/* FP:mod.rs-0195 */     /// Number of SCCs,
/* FP:mod.rs-0196 */     fn len(&self) -> usize {
/* FP:mod.rs-0197 */         self.scc_details.len()
/* FP:mod.rs-0198 */     }
/* FP:mod.rs-0199 */ 
/* FP:mod.rs-0200 */     /// Returns the successors of the given SCC.
/* FP:mod.rs-0201 */     fn successors(&self, scc: S) -> &[S] {
/* FP:mod.rs-0202 */         &self.all_successors[self.scc_details[scc].range.clone()]
/* FP:mod.rs-0203 */     }
/* FP:mod.rs-0204 */ 
/* FP:mod.rs-0205 */     /// Creates a new SCC with `successors` as its successors and
/* FP:mod.rs-0206 */     /// returns the resulting index.
/* FP:mod.rs-0207 */     fn create_scc(&mut self, successors: impl IntoIterator<Item = S>) -> S {
/* FP:mod.rs-0208 */         // Store the successors on `scc_successors_vec`, remembering
/* FP:mod.rs-0209 */         // the range of indices.
/* FP:mod.rs-0210 */         let all_successors_start = self.all_successors.len();
/* FP:mod.rs-0211 */         self.all_successors.extend(successors);
/* FP:mod.rs-0212 */         let all_successors_end = self.all_successors.len();
/* FP:mod.rs-0213 */ 
/* FP:mod.rs-0214 */         debug!(
/* FP:mod.rs-0215 */             "create_scc({:?}) successors={:?}",
/* FP:mod.rs-0216 */             self.len(),
/* FP:mod.rs-0217 */             &self.all_successors[all_successors_start..all_successors_end],
/* FP:mod.rs-0218 */         );
/* FP:mod.rs-0219 */ 
/* FP:mod.rs-0220 */         let range = all_successors_start..all_successors_end;
/* FP:mod.rs-0221 */         let metadata = SccDetails { range };
/* FP:mod.rs-0222 */         self.scc_details.push(metadata)
/* FP:mod.rs-0223 */     }
/* FP:mod.rs-0224 */ }
/* FP:mod.rs-0225 */ 
/* FP:mod.rs-0226 */ struct SccsConstruction<'c, 'a, G, A>
/* FP:mod.rs-0227 */ where
/* FP:mod.rs-0228 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0229 */     A: Annotations<G::Node>,
/* FP:mod.rs-0230 */ {
/* FP:mod.rs-0231 */     graph: &'c G,
/* FP:mod.rs-0232 */ 
/* FP:mod.rs-0233 */     /// The state of each node; used during walk to record the stack
/* FP:mod.rs-0234 */     /// and after walk to record what cycle each node ended up being
/* FP:mod.rs-0235 */     /// in.
/* FP:mod.rs-0236 */     node_states: IndexVec<G::Node, NodeState<G::Node, A::SccIdx, A::Ann>>,
/* FP:mod.rs-0237 */ 
/* FP:mod.rs-0238 */     /// The stack of nodes that we are visiting as part of the DFS.
/* FP:mod.rs-0239 */     node_stack: Vec<G::Node>,
/* FP:mod.rs-0240 */ 
/* FP:mod.rs-0241 */     /// The stack of successors: as we visit a node, we mark our
/* FP:mod.rs-0242 */     /// position in this stack, and when we encounter a successor SCC,
/* FP:mod.rs-0243 */     /// we push it on the stack. When we complete an SCC, we can pop
/* FP:mod.rs-0244 */     /// everything off the stack that was found along the way.
/* FP:mod.rs-0245 */     successors_stack: Vec<A::SccIdx>,
/* FP:mod.rs-0246 */ 
/* FP:mod.rs-0247 */     /// A set used to strip duplicates. As we accumulate successors
/* FP:mod.rs-0248 */     /// into the successors_stack, we sometimes get duplicate entries.
/* FP:mod.rs-0249 */     /// We use this set to remove those -- we also keep its storage
/* FP:mod.rs-0250 */     /// around between successors to amortize memory allocation costs.
/* FP:mod.rs-0251 */     duplicate_set: FxHashSet<A::SccIdx>,
/* FP:mod.rs-0252 */ 
/* FP:mod.rs-0253 */     scc_data: SccData<A::SccIdx>,
/* FP:mod.rs-0254 */ 
/* FP:mod.rs-0255 */     annotations: &'a mut A,
/* FP:mod.rs-0256 */ }
/* FP:mod.rs-0257 */ 
/* FP:mod.rs-0258 */ #[derive(Copy, Clone, Debug)]
/* FP:mod.rs-0259 */ enum NodeState<N, S, A: Annotation> {
/* FP:mod.rs-0260 */     /// This node has not yet been visited as part of the DFS.
/* FP:mod.rs-0261 */     ///
/* FP:mod.rs-0262 */     /// After SCC construction is complete, this state ought to be
/* FP:mod.rs-0263 */     /// impossible.
/* FP:mod.rs-0264 */     NotVisited,
/* FP:mod.rs-0265 */ 
/* FP:mod.rs-0266 */     /// This node is currently being walked as part of our DFS. It is on
/* FP:mod.rs-0267 */     /// the stack at the depth `depth` and its current annotation is
/* FP:mod.rs-0268 */     /// `annotation`.
/* FP:mod.rs-0269 */     ///
/* FP:mod.rs-0270 */     /// After SCC construction is complete, this state ought to be
/* FP:mod.rs-0271 */     /// impossible.
/* FP:mod.rs-0272 */     BeingVisited { depth: usize, annotation: A },
/* FP:mod.rs-0273 */ 
/* FP:mod.rs-0274 */     /// Indicates that this node is a member of the given cycle where
/* FP:mod.rs-0275 */     /// the merged annotation is `annotation`.
/* FP:mod.rs-0276 */     /// Note that an SCC can have several cycles, so its final annotation
/* FP:mod.rs-0277 */     /// is the merged value of all its member annotations.
/* FP:mod.rs-0278 */     InCycle { scc_index: S, annotation: A },
/* FP:mod.rs-0279 */ 
/* FP:mod.rs-0280 */     /// Indicates that this node is a member of whatever cycle
/* FP:mod.rs-0281 */     /// `parent` is a member of. This state is transient: whenever we
/* FP:mod.rs-0282 */     /// see it, we try to overwrite it with the current state of
/* FP:mod.rs-0283 */     /// `parent` (this is the "path compression" step of a union-find
/* FP:mod.rs-0284 */     /// algorithm).
/* FP:mod.rs-0285 */     InCycleWith { parent: N },
/* FP:mod.rs-0286 */ }
/* FP:mod.rs-0287 */ 
/* FP:mod.rs-0288 */ /// The state of walking a given node.
/* FP:mod.rs-0289 */ #[derive(Copy, Clone, Debug)]
/* FP:mod.rs-0290 */ enum WalkReturn<S, A: Annotation> {
/* FP:mod.rs-0291 */     /// The walk found a cycle, but the entire component is not known to have
/* FP:mod.rs-0292 */     /// been fully walked yet. We only know the minimum depth of  this
/* FP:mod.rs-0293 */     /// component in a minimum spanning tree of the graph. This component
/* FP:mod.rs-0294 */     /// is tentatively represented by the state of the first node of this
/* FP:mod.rs-0295 */     /// cycle we met, which is at `min_depth`.
/* FP:mod.rs-0296 */     Cycle { min_depth: usize, annotation: A },
/* FP:mod.rs-0297 */     /// The SCC and everything reachable from it have been fully walked.
/* FP:mod.rs-0298 */     /// At this point we know what is inside the SCC as we have visited every
/* FP:mod.rs-0299 */     /// node reachable from it. The SCC can now be fully represented by its ID.
/* FP:mod.rs-0300 */     Complete { scc_index: S, annotation: A },
/* FP:mod.rs-0301 */ }
/* FP:mod.rs-0302 */ 
/* FP:mod.rs-0303 */ impl<'c, 'a, G, A> SccsConstruction<'c, 'a, G, A>
/* FP:mod.rs-0304 */ where
/* FP:mod.rs-0305 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0306 */     A: Annotations<G::Node>,
/* FP:mod.rs-0307 */ {
/* FP:mod.rs-0308 */     /// Identifies SCCs in the graph `G` and computes the resulting
/* FP:mod.rs-0309 */     /// DAG. This uses a variant of [Tarjan's
/* FP:mod.rs-0310 */     /// algorithm][wikipedia]. The high-level summary of the algorithm
/* FP:mod.rs-0311 */     /// is that we do a depth-first search. Along the way, we keep a
/* FP:mod.rs-0312 */     /// stack of each node whose successors are being visited. We
/* FP:mod.rs-0313 */     /// track the depth of each node on this stack (there is no depth
/* FP:mod.rs-0314 */     /// if the node is not on the stack). When we find that some node
/* FP:mod.rs-0315 */     /// N with depth D can reach some other node N' with lower depth
/* FP:mod.rs-0316 */     /// D' (i.e., D' < D), we know that N, N', and all nodes in
/* FP:mod.rs-0317 */     /// between them on the stack are part of an SCC.
/* FP:mod.rs-0318 */     ///
/* FP:mod.rs-0319 */     /// Additionally, we keep track of a current annotation of the SCC.
/* FP:mod.rs-0320 */     ///
/* FP:mod.rs-0321 */     /// [wikipedia]: https://bit.ly/2EZIx84
/* FP:mod.rs-0322 */     fn construct(graph: &'c G, annotations: &'a mut A) -> Sccs<G::Node, A::SccIdx> {
/* FP:mod.rs-0323 */         let num_nodes = graph.num_nodes();
/* FP:mod.rs-0324 */ 
/* FP:mod.rs-0325 */         let mut this = Self {
/* FP:mod.rs-0326 */             graph,
/* FP:mod.rs-0327 */             node_states: IndexVec::from_elem_n(NodeState::NotVisited, num_nodes),
/* FP:mod.rs-0328 */             node_stack: Vec::with_capacity(num_nodes),
/* FP:mod.rs-0329 */             successors_stack: Vec::new(),
/* FP:mod.rs-0330 */             scc_data: SccData { scc_details: IndexVec::new(), all_successors: Vec::new() },
/* FP:mod.rs-0331 */             duplicate_set: FxHashSet::default(),
/* FP:mod.rs-0332 */             annotations,
/* FP:mod.rs-0333 */         };
/* FP:mod.rs-0334 */ 
/* FP:mod.rs-0335 */         let scc_indices = graph
/* FP:mod.rs-0336 */             .iter_nodes()
/* FP:mod.rs-0337 */             .map(|node| match this.start_walk_from(node) {
/* FP:mod.rs-0338 */                 WalkReturn::Complete { scc_index, .. } => scc_index,
/* FP:mod.rs-0339 */                 WalkReturn::Cycle { min_depth, .. } => {
/* FP:mod.rs-0340 */                     panic!("`start_walk_node({node:?})` returned cycle with depth {min_depth:?}")
/* FP:mod.rs-0341 */                 }
/* FP:mod.rs-0342 */             })
/* FP:mod.rs-0343 */             .collect();
/* FP:mod.rs-0344 */ 
/* FP:mod.rs-0345 */         Sccs { scc_indices, scc_data: this.scc_data }
/* FP:mod.rs-0346 */     }
/* FP:mod.rs-0347 */ 
/* FP:mod.rs-0348 */     fn start_walk_from(&mut self, node: G::Node) -> WalkReturn<A::SccIdx, A::Ann> {
/* FP:mod.rs-0349 */         self.inspect_node(node).unwrap_or_else(|| self.walk_unvisited_node(node))
/* FP:mod.rs-0350 */     }
/* FP:mod.rs-0351 */ 
/* FP:mod.rs-0352 */     /// Inspect a node during the DFS. We first examine its current
/* FP:mod.rs-0353 */     /// state -- if it is not yet visited (`NotVisited`), return `None` so
/* FP:mod.rs-0354 */     /// that the caller might push it onto the stack and start walking its
/* FP:mod.rs-0355 */     /// successors.
/* FP:mod.rs-0356 */     ///
/* FP:mod.rs-0357 */     /// If it is already on the DFS stack it will be in the state
/* FP:mod.rs-0358 */     /// `BeingVisited`. In that case, we have found a cycle and we
/* FP:mod.rs-0359 */     /// return the depth from the stack.
/* FP:mod.rs-0360 */     ///
/* FP:mod.rs-0361 */     /// Otherwise, we are looking at a node that has already been
/* FP:mod.rs-0362 */     /// completely visited. We therefore return `WalkReturn::Complete`
/* FP:mod.rs-0363 */     /// with its associated SCC index.
/* FP:mod.rs-0364 */     fn inspect_node(&mut self, node: G::Node) -> Option<WalkReturn<A::SccIdx, A::Ann>> {
/* FP:mod.rs-0365 */         Some(match self.find_state(node) {
/* FP:mod.rs-0366 */             NodeState::InCycle { scc_index, annotation } => {
/* FP:mod.rs-0367 */                 WalkReturn::Complete { scc_index, annotation }
/* FP:mod.rs-0368 */             }
/* FP:mod.rs-0369 */ 
/* FP:mod.rs-0370 */             NodeState::BeingVisited { depth: min_depth, annotation } => {
/* FP:mod.rs-0371 */                 WalkReturn::Cycle { min_depth, annotation }
/* FP:mod.rs-0372 */             }
/* FP:mod.rs-0373 */ 
/* FP:mod.rs-0374 */             NodeState::NotVisited => return None,
/* FP:mod.rs-0375 */ 
/* FP:mod.rs-0376 */             NodeState::InCycleWith { parent } => panic!(
/* FP:mod.rs-0377 */                 "`find_state` returned `InCycleWith({parent:?})`, which ought to be impossible"
/* FP:mod.rs-0378 */             ),
/* FP:mod.rs-0379 */         })
/* FP:mod.rs-0380 */     }
/* FP:mod.rs-0381 */ 
/* FP:mod.rs-0382 */     /// Fetches the state of the node `r`. If `r` is recorded as being
/* FP:mod.rs-0383 */     /// in a cycle with some other node `r2`, then fetches the state
/* FP:mod.rs-0384 */     /// of `r2` (and updates `r` to reflect current result). This is
/* FP:mod.rs-0385 */     /// basically the "find" part of a standard union-find algorithm
/* FP:mod.rs-0386 */     /// (with path compression).
/* FP:mod.rs-0387 */     fn find_state(&mut self, mut node: G::Node) -> NodeState<G::Node, A::SccIdx, A::Ann> {
/* FP:mod.rs-0388 */         // To avoid recursion we temporarily reuse the `parent` of each
/* FP:mod.rs-0389 */         // InCycleWith link to encode a downwards link while compressing
/* FP:mod.rs-0390 */         // the path. After we have found the root or deepest node being
/* FP:mod.rs-0391 */         // visited, we traverse the reverse links and correct the node
/* FP:mod.rs-0392 */         // states on the way.
/* FP:mod.rs-0393 */         //
/* FP:mod.rs-0394 */         // **Note**: This mutation requires that this is a leaf function
/* FP:mod.rs-0395 */         // or at least that none of the called functions inspects the
/* FP:mod.rs-0396 */         // current node states. Luckily, we are a leaf.
/* FP:mod.rs-0397 */ 
/* FP:mod.rs-0398 */         // Remember one previous link. The termination condition when
/* FP:mod.rs-0399 */         // following links downwards is then simply as soon as we have
/* FP:mod.rs-0400 */         // found the initial self-loop.
/* FP:mod.rs-0401 */         let mut previous_node = node;
/* FP:mod.rs-0402 */ 
/* FP:mod.rs-0403 */         // Ultimately propagated to all the transitive parents when following
/* FP:mod.rs-0404 */         // `InCycleWith` upwards.
/* FP:mod.rs-0405 */         // This loop performs the downward link encoding mentioned above. Details below!
/* FP:mod.rs-0406 */         // Note that there are two different states being assigned: the root state, and
/* FP:mod.rs-0407 */         // a potentially derived version of the root state for non-root nodes in the chain.
/* FP:mod.rs-0408 */         let (root_state, assigned_state) = {
/* FP:mod.rs-0409 */             loop {
/* FP:mod.rs-0410 */                 trace!("find_state(r = {node:?} in state {:?})", self.node_states[node]);
/* FP:mod.rs-0411 */                 match self.node_states[node] {
/* FP:mod.rs-0412 */                     // This must have been the first and only state since it is unexplored*;
/* FP:mod.rs-0413 */                     // no update needed! * Unless there is a bug :')
/* FP:mod.rs-0414 */                     s @ NodeState::NotVisited => return s,
/* FP:mod.rs-0415 */                     // We are in a completely discovered SCC; every node on our path is in that SCC:
/* FP:mod.rs-0416 */                     s @ NodeState::InCycle { .. } => break (s, s),
/* FP:mod.rs-0417 */                     // The Interesting Third Base Case: we are a path back to a root node
/* FP:mod.rs-0418 */                     // still being explored. Now we need that node to keep its state and
/* FP:mod.rs-0419 */                     // every other node to be recorded as being in whatever component that
/* FP:mod.rs-0420 */                     // ends up in.
/* FP:mod.rs-0421 */                     s @ NodeState::BeingVisited { depth, .. } => {
/* FP:mod.rs-0422 */                         break (s, NodeState::InCycleWith { parent: self.node_stack[depth] });
/* FP:mod.rs-0423 */                     }
/* FP:mod.rs-0424 */                     // We are not at the head of a path; keep compressing it!
/* FP:mod.rs-0425 */                     NodeState::InCycleWith { parent } => {
/* FP:mod.rs-0426 */                         // We test this, to be extremely sure that we never
/* FP:mod.rs-0427 */                         // ever break our termination condition for the
/* FP:mod.rs-0428 */                         // reverse iteration loop.
/* FP:mod.rs-0429 */                         assert!(node != parent, "Node can not be in cycle with itself");
/* FP:mod.rs-0430 */ 
/* FP:mod.rs-0431 */                         // Store the previous node as an inverted list link
/* FP:mod.rs-0432 */                         self.node_states[node] = NodeState::InCycleWith { parent: previous_node };
/* FP:mod.rs-0433 */                         // Update to parent node.
/* FP:mod.rs-0434 */                         previous_node = node;
/* FP:mod.rs-0435 */                         node = parent;
/* FP:mod.rs-0436 */                     }
/* FP:mod.rs-0437 */                 }
/* FP:mod.rs-0438 */             }
/* FP:mod.rs-0439 */         };
/* FP:mod.rs-0440 */ 
/* FP:mod.rs-0441 */         // The states form a graph where up to one outgoing link is stored at
/* FP:mod.rs-0442 */         // each node. Initially in general,
/* FP:mod.rs-0443 */         //
/* FP:mod.rs-0444 */         //                                                  E
/* FP:mod.rs-0445 */         //                                                  ^
/* FP:mod.rs-0446 */         //                                                  |
/* FP:mod.rs-0447 */         //                                InCycleWith/BeingVisited/NotVisited
/* FP:mod.rs-0448 */         //                                                  |
/* FP:mod.rs-0449 */         //   A-InCycleWith->B-InCycleWith…>C-InCycleWith->D-+
/* FP:mod.rs-0450 */         //   |
/* FP:mod.rs-0451 */         //   = node, previous_node
/* FP:mod.rs-0452 */         //
/* FP:mod.rs-0453 */         // After the first loop, this will look like
/* FP:mod.rs-0454 */         //                                                  E
/* FP:mod.rs-0455 */         //                                                  ^
/* FP:mod.rs-0456 */         //                                                  |
/* FP:mod.rs-0457 */         //                                InCycleWith/BeingVisited/NotVisited
/* FP:mod.rs-0458 */         //                                                  |
/* FP:mod.rs-0459 */         // +>A<-InCycleWith-B<…InCycleWith-C<-InCycleWith-D-+
/* FP:mod.rs-0460 */         // | |                             |              |
/* FP:mod.rs-0461 */         // | InCycleWith                   |              = node
/* FP:mod.rs-0462 */         // +-+                             =previous_node
/* FP:mod.rs-0463 */         //
/* FP:mod.rs-0464 */         // Note in particular that A will be linked to itself in a self-cycle
/* FP:mod.rs-0465 */         // and no other self-cycles occur due to how InCycleWith is assigned in
/* FP:mod.rs-0466 */         // the find phase implemented by `walk_unvisited_node`.
/* FP:mod.rs-0467 */         //
/* FP:mod.rs-0468 */         // We now want to compress the path, that is assign the state of the
/* FP:mod.rs-0469 */         // link D-E to all other links.
/* FP:mod.rs-0470 */         //
/* FP:mod.rs-0471 */         // We can then walk backwards, starting from `previous_node`, and assign
/* FP:mod.rs-0472 */         // each node in the list with the updated state. The loop terminates
/* FP:mod.rs-0473 */         // when we reach the self-cycle.
/* FP:mod.rs-0474 */ 
/* FP:mod.rs-0475 */         // Move backwards until we found the node where we started. We
/* FP:mod.rs-0476 */         // will know when we hit the state where previous_node == node.
/* FP:mod.rs-0477 */         loop {
/* FP:mod.rs-0478 */             // Back at the beginning, we can return. Note that we return the root state.
/* FP:mod.rs-0479 */             // This is because for components being explored, we would otherwise get a
/* FP:mod.rs-0480 */             // `node_state[n] = InCycleWith{ parent: n }` and that's wrong.
/* FP:mod.rs-0481 */             if previous_node == node {
/* FP:mod.rs-0482 */                 return root_state;
/* FP:mod.rs-0483 */             }
/* FP:mod.rs-0484 */             trace!("Compressing {node:?} down to {previous_node:?} with state {assigned_state:?}");
/* FP:mod.rs-0485 */ 
/* FP:mod.rs-0486 */             // Update to previous node in the link.
/* FP:mod.rs-0487 */             match self.node_states[previous_node] {
/* FP:mod.rs-0488 */                 NodeState::InCycleWith { parent: previous } => {
/* FP:mod.rs-0489 */                     node = previous_node;
/* FP:mod.rs-0490 */                     previous_node = previous;
/* FP:mod.rs-0491 */                 }
/* FP:mod.rs-0492 */                 // Only InCycleWith nodes were added to the reverse linked list.
/* FP:mod.rs-0493 */                 other => unreachable!("Invalid previous link while compressing cycle: {other:?}"),
/* FP:mod.rs-0494 */             }
/* FP:mod.rs-0495 */ 
/* FP:mod.rs-0496 */             // Update the node state to the (potentially derived) state.
/* FP:mod.rs-0497 */             // If the root is still being explored, this is
/* FP:mod.rs-0498 */             // `InCycleWith{ parent: <root node>}`, otherwise
/* FP:mod.rs-0499 */             // `assigned_state == root_state`.
/* FP:mod.rs-0500 */             self.node_states[node] = assigned_state;
/* FP:mod.rs-0501 */         }
/* FP:mod.rs-0502 */     }
/* FP:mod.rs-0503 */ 
/* FP:mod.rs-0504 */     /// Walks a node that has never been visited before.
/* FP:mod.rs-0505 */     ///
/* FP:mod.rs-0506 */     /// Call this method when `inspect_node` has returned `None`. Having the
/* FP:mod.rs-0507 */     /// caller decide avoids mutual recursion between the two methods and allows
/* FP:mod.rs-0508 */     /// us to maintain an allocated stack for nodes on the path between calls.
/* FP:mod.rs-0509 */     #[instrument(skip(self, initial), level = "trace")]
/* FP:mod.rs-0510 */     fn walk_unvisited_node(&mut self, initial: G::Node) -> WalkReturn<A::SccIdx, A::Ann> {
/* FP:mod.rs-0511 */         trace!("Walk unvisited node: {initial:?}");
/* FP:mod.rs-0512 */         struct VisitingNodeFrame<G: DirectedGraph, Successors, A> {
/* FP:mod.rs-0513 */             node: G::Node,
/* FP:mod.rs-0514 */             successors: Option<Successors>,
/* FP:mod.rs-0515 */             depth: usize,
/* FP:mod.rs-0516 */             min_depth: usize,
/* FP:mod.rs-0517 */             successors_len: usize,
/* FP:mod.rs-0518 */             min_cycle_root: G::Node,
/* FP:mod.rs-0519 */             successor_node: G::Node,
/* FP:mod.rs-0520 */             /// The annotation for the SCC starting in `node`. It may or may
/* FP:mod.rs-0521 */             /// not contain other nodes.
/* FP:mod.rs-0522 */             current_component_annotation: A,
/* FP:mod.rs-0523 */         }
/* FP:mod.rs-0524 */ 
/* FP:mod.rs-0525 */         // Move the stack to a local variable. We want to utilize the existing allocation and
/* FP:mod.rs-0526 */         // mutably borrow it without borrowing self at the same time.
/* FP:mod.rs-0527 */         let mut successors_stack = core::mem::take(&mut self.successors_stack);
/* FP:mod.rs-0528 */ 
/* FP:mod.rs-0529 */         debug_assert_eq!(successors_stack.len(), 0);
/* FP:mod.rs-0530 */ 
/* FP:mod.rs-0531 */         let mut stack: Vec<VisitingNodeFrame<G, _, _>> = vec![VisitingNodeFrame {
/* FP:mod.rs-0532 */             node: initial,
/* FP:mod.rs-0533 */             depth: 0,
/* FP:mod.rs-0534 */             min_depth: 0,
/* FP:mod.rs-0535 */             successors: None,
/* FP:mod.rs-0536 */             successors_len: 0,
/* FP:mod.rs-0537 */             min_cycle_root: initial,
/* FP:mod.rs-0538 */             successor_node: initial,
/* FP:mod.rs-0539 */             current_component_annotation: self.annotations.new(initial),
/* FP:mod.rs-0540 */         }];
/* FP:mod.rs-0541 */ 
/* FP:mod.rs-0542 */         let mut return_value = None;
/* FP:mod.rs-0543 */ 
/* FP:mod.rs-0544 */         'recurse: while let Some(frame) = stack.last_mut() {
/* FP:mod.rs-0545 */             let VisitingNodeFrame {
/* FP:mod.rs-0546 */                 node,
/* FP:mod.rs-0547 */                 depth,
/* FP:mod.rs-0548 */                 successors,
/* FP:mod.rs-0549 */                 successors_len,
/* FP:mod.rs-0550 */                 min_depth,
/* FP:mod.rs-0551 */                 min_cycle_root,
/* FP:mod.rs-0552 */                 successor_node,
/* FP:mod.rs-0553 */                 current_component_annotation,
/* FP:mod.rs-0554 */             } = frame;
/* FP:mod.rs-0555 */             let node = *node;
/* FP:mod.rs-0556 */             let depth = *depth;
/* FP:mod.rs-0557 */ 
/* FP:mod.rs-0558 */             trace!(
/* FP:mod.rs-0559 */                 "Visiting {node:?} at depth {depth:?}, annotation: {current_component_annotation:?}"
/* FP:mod.rs-0560 */             );
/* FP:mod.rs-0561 */ 
/* FP:mod.rs-0562 */             let successors = match successors {
/* FP:mod.rs-0563 */                 Some(successors) => successors,
/* FP:mod.rs-0564 */                 None => {
/* FP:mod.rs-0565 */                     // This None marks that we still have the initialize this node's frame.
/* FP:mod.rs-0566 */                     trace!(?depth, ?node);
/* FP:mod.rs-0567 */ 
/* FP:mod.rs-0568 */                     debug_assert_matches!(self.node_states[node], NodeState::NotVisited);
/* FP:mod.rs-0569 */ 
/* FP:mod.rs-0570 */                     // Push `node` onto the stack.
/* FP:mod.rs-0571 */                     self.node_states[node] = NodeState::BeingVisited {
/* FP:mod.rs-0572 */                         depth,
/* FP:mod.rs-0573 */                         annotation: *current_component_annotation,
/* FP:mod.rs-0574 */                     };
/* FP:mod.rs-0575 */                     self.node_stack.push(node);
/* FP:mod.rs-0576 */ 
/* FP:mod.rs-0577 */                     // Walk each successor of the node, looking to see if any of
/* FP:mod.rs-0578 */                     // them can reach a node that is presently on the stack. If
/* FP:mod.rs-0579 */                     // so, that means they can also reach us.
/* FP:mod.rs-0580 */                     *successors_len = successors_stack.len();
/* FP:mod.rs-0581 */                     // Set and return a reference, this is currently empty.
/* FP:mod.rs-0582 */                     successors.get_or_insert(self.graph.successors(node))
/* FP:mod.rs-0583 */                 }
/* FP:mod.rs-0584 */             };
/* FP:mod.rs-0585 */ 
/* FP:mod.rs-0586 */             // Now that the successors iterator is initialized, this is a constant for this frame.
/* FP:mod.rs-0587 */             let successors_len = *successors_len;
/* FP:mod.rs-0588 */ 
/* FP:mod.rs-0589 */             // Construct iterators for the nodes and walk results. There are two cases:
/* FP:mod.rs-0590 */             // * The walk of a successor node returned.
/* FP:mod.rs-0591 */             // * The remaining successor nodes.
/* FP:mod.rs-0592 */             let returned_walk =
/* FP:mod.rs-0593 */                 return_value.take().into_iter().map(|walk| (*successor_node, Some(walk)));
/* FP:mod.rs-0594 */ 
/* FP:mod.rs-0595 */             let successor_walk = successors.map(|successor_node| {
/* FP:mod.rs-0596 */                 trace!(?node, ?successor_node);
/* FP:mod.rs-0597 */                 (successor_node, self.inspect_node(successor_node))
/* FP:mod.rs-0598 */             });
/* FP:mod.rs-0599 */             for (successor_node, walk) in returned_walk.chain(successor_walk) {
/* FP:mod.rs-0600 */                 match walk {
/* FP:mod.rs-0601 */                     // The starting node `node` leads to a cycle whose earliest node,
/* FP:mod.rs-0602 */                     // `successor_node`, is at `min_depth`. There may be more cycles.
/* FP:mod.rs-0603 */                     Some(WalkReturn::Cycle {
/* FP:mod.rs-0604 */                         min_depth: successor_min_depth,
/* FP:mod.rs-0605 */                         annotation: successor_annotation,
/* FP:mod.rs-0606 */                     }) => {
/* FP:mod.rs-0607 */                         trace!(
/* FP:mod.rs-0608 */                             "Cycle found from {node:?}, minimum depth: {successor_min_depth:?}, annotation: {successor_annotation:?}"
/* FP:mod.rs-0609 */                         );
/* FP:mod.rs-0610 */                         // Track the minimum depth we can reach.
/* FP:mod.rs-0611 */                         assert!(successor_min_depth <= depth);
/* FP:mod.rs-0612 */                         if successor_min_depth < *min_depth {
/* FP:mod.rs-0613 */                             trace!(?node, ?successor_min_depth);
/* FP:mod.rs-0614 */                             *min_depth = successor_min_depth;
/* FP:mod.rs-0615 */                             *min_cycle_root = successor_node;
/* FP:mod.rs-0616 */                         }
/* FP:mod.rs-0617 */                         current_component_annotation.update_scc(successor_annotation);
/* FP:mod.rs-0618 */                     }
/* FP:mod.rs-0619 */                     // The starting node `node` is succeeded by a fully identified SCC
/* FP:mod.rs-0620 */                     // which is now added to the set under `scc_index`.
/* FP:mod.rs-0621 */                     Some(WalkReturn::Complete {
/* FP:mod.rs-0622 */                         scc_index: successor_scc_index,
/* FP:mod.rs-0623 */                         annotation: successor_annotation,
/* FP:mod.rs-0624 */                     }) => {
/* FP:mod.rs-0625 */                         trace!(
/* FP:mod.rs-0626 */                             "Complete; {node:?} is root of complete-visited SCC idx {successor_scc_index:?} with annotation {successor_annotation:?}"
/* FP:mod.rs-0627 */                         );
/* FP:mod.rs-0628 */                         // Push the completed SCC indices onto
/* FP:mod.rs-0629 */                         // the `successors_stack` for later.
/* FP:mod.rs-0630 */                         trace!(?node, ?successor_scc_index);
/* FP:mod.rs-0631 */                         successors_stack.push(successor_scc_index);
/* FP:mod.rs-0632 */                         current_component_annotation.update_reachable(successor_annotation);
/* FP:mod.rs-0633 */                     }
/* FP:mod.rs-0634 */                     // `node` has no more (direct) successors; search recursively.
/* FP:mod.rs-0635 */                     None => {
/* FP:mod.rs-0636 */                         let depth = depth + 1;
/* FP:mod.rs-0637 */                         trace!("Recursing down into {successor_node:?} at depth {depth:?}");
/* FP:mod.rs-0638 */                         trace!(?depth, ?successor_node);
/* FP:mod.rs-0639 */                         // Remember which node the return value will come from.
/* FP:mod.rs-0640 */                         frame.successor_node = successor_node;
/* FP:mod.rs-0641 */                         // Start a new stack frame, then step into it.
/* FP:mod.rs-0642 */                         stack.push(VisitingNodeFrame {
/* FP:mod.rs-0643 */                             node: successor_node,
/* FP:mod.rs-0644 */                             depth,
/* FP:mod.rs-0645 */                             successors: None,
/* FP:mod.rs-0646 */                             successors_len: 0,
/* FP:mod.rs-0647 */                             min_depth: depth,
/* FP:mod.rs-0648 */                             min_cycle_root: successor_node,
/* FP:mod.rs-0649 */                             successor_node,
/* FP:mod.rs-0650 */                             current_component_annotation: self.annotations.new(successor_node),
/* FP:mod.rs-0651 */                         });
/* FP:mod.rs-0652 */                         continue 'recurse;
/* FP:mod.rs-0653 */                     }
/* FP:mod.rs-0654 */                 }
/* FP:mod.rs-0655 */             }
/* FP:mod.rs-0656 */ 
/* FP:mod.rs-0657 */             trace!("Finished walk from {node:?} with annotation: {current_component_annotation:?}");
/* FP:mod.rs-0658 */ 
/* FP:mod.rs-0659 */             // Completed walk, remove `node` from the stack.
/* FP:mod.rs-0660 */             let r = self.node_stack.pop();
/* FP:mod.rs-0661 */             debug_assert_eq!(r, Some(node));
/* FP:mod.rs-0662 */ 
/* FP:mod.rs-0663 */             // Remove the frame, it's done.
/* FP:mod.rs-0664 */             let frame = stack.pop().unwrap();
/* FP:mod.rs-0665 */             let current_component_annotation = frame.current_component_annotation;
/* FP:mod.rs-0666 */             debug_assert_eq!(frame.node, node);
/* FP:mod.rs-0667 */ 
/* FP:mod.rs-0668 */             // If `min_depth == depth`, then we are the root of the
/* FP:mod.rs-0669 */             // cycle: we can't reach anyone further down the stack.
/* FP:mod.rs-0670 */ 
/* FP:mod.rs-0671 */             // Pass the 'return value' down the stack.
/* FP:mod.rs-0672 */             // We return one frame at a time so there can't be another return value.
/* FP:mod.rs-0673 */             debug_assert!(return_value.is_none());
/* FP:mod.rs-0674 */             return_value = Some(if frame.min_depth == depth {
/* FP:mod.rs-0675 */                 // We are at the head of the component.
/* FP:mod.rs-0676 */ 
/* FP:mod.rs-0677 */                 // Note that successor stack may have duplicates, so we
/* FP:mod.rs-0678 */                 // want to remove those:
/* FP:mod.rs-0679 */                 let deduplicated_successors = {
/* FP:mod.rs-0680 */                     let duplicate_set = &mut self.duplicate_set;
/* FP:mod.rs-0681 */                     duplicate_set.clear();
/* FP:mod.rs-0682 */                     successors_stack
/* FP:mod.rs-0683 */                         .drain(successors_len..)
/* FP:mod.rs-0684 */                         .filter(move |&i| duplicate_set.insert(i))
/* FP:mod.rs-0685 */                 };
/* FP:mod.rs-0686 */ 
/* FP:mod.rs-0687 */                 debug!("Creating SCC rooted in {node:?} with successor {:?}", frame.successor_node);
/* FP:mod.rs-0688 */ 
/* FP:mod.rs-0689 */                 let scc_index = self.scc_data.create_scc(deduplicated_successors);
/* FP:mod.rs-0690 */ 
/* FP:mod.rs-0691 */                 self.annotations.annotate_scc(scc_index, current_component_annotation);
/* FP:mod.rs-0692 */ 
/* FP:mod.rs-0693 */                 self.node_states[node] =
/* FP:mod.rs-0694 */                     NodeState::InCycle { scc_index, annotation: current_component_annotation };
/* FP:mod.rs-0695 */ 
/* FP:mod.rs-0696 */                 WalkReturn::Complete { scc_index, annotation: current_component_annotation }
/* FP:mod.rs-0697 */             } else {
/* FP:mod.rs-0698 */                 // We are not the head of the cycle. Return back to our
/* FP:mod.rs-0699 */                 // caller. They will take ownership of the
/* FP:mod.rs-0700 */                 // `self.successors` data that we pushed.
/* FP:mod.rs-0701 */                 self.node_states[node] = NodeState::InCycleWith { parent: frame.min_cycle_root };
/* FP:mod.rs-0702 */                 WalkReturn::Cycle {
/* FP:mod.rs-0703 */                     min_depth: frame.min_depth,
/* FP:mod.rs-0704 */                     annotation: current_component_annotation,
/* FP:mod.rs-0705 */                 }
/* FP:mod.rs-0706 */             });
/* FP:mod.rs-0707 */         }
/* FP:mod.rs-0708 */ 
/* FP:mod.rs-0709 */         // Keep the allocation we used for successors_stack.
/* FP:mod.rs-0710 */         self.successors_stack = successors_stack;
/* FP:mod.rs-0711 */         debug_assert_eq!(self.successors_stack.len(), 0);
/* FP:mod.rs-0712 */ 
/* FP:mod.rs-0713 */         return_value.unwrap()
/* FP:mod.rs-0714 */     }
/* FP:mod.rs-0715 */ }