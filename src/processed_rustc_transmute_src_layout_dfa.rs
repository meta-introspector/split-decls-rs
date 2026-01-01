/* FP:dfa.rs-0001 */ use std::fmt;
/* FP:dfa.rs-0002 */ use std::iter::Peekable;
/* FP:dfa.rs-0003 */ use std::sync::atomic::{AtomicU32, Ordering};
/* FP:dfa.rs-0004 */ 
/* FP:dfa.rs-0005 */ use super::{Byte, Reference, Region, Tree, Type, Uninhabited};
/* FP:dfa.rs-0006 */ use crate::{Map, Set};
/* FP:dfa.rs-0007 */ 
/* FP:dfa.rs-0008 */ #[derive(PartialEq)]
/* FP:dfa.rs-0009 */ #[cfg_attr(test, derive(Clone))]
/* FP:dfa.rs-0010 */ pub(crate) struct Dfa<R, T>
/* FP:dfa.rs-0011 */ where
/* FP:dfa.rs-0012 */     R: Region,
/* FP:dfa.rs-0013 */     T: Type,
/* FP:dfa.rs-0014 */ {
/* FP:dfa.rs-0015 */     pub(crate) transitions: Map<State, Transitions<R, T>>,
/* FP:dfa.rs-0016 */     pub(crate) start: State,
/* FP:dfa.rs-0017 */     pub(crate) accept: State,
/* FP:dfa.rs-0018 */ }
/* FP:dfa.rs-0019 */ 
/* FP:dfa.rs-0020 */ #[derive(PartialEq, Clone, Debug)]
/* FP:dfa.rs-0021 */ pub(crate) struct Transitions<R, T>
/* FP:dfa.rs-0022 */ where
/* FP:dfa.rs-0023 */     R: Region,
/* FP:dfa.rs-0024 */     T: Type,
/* FP:dfa.rs-0025 */ {
/* FP:dfa.rs-0026 */     byte_transitions: EdgeSet<State>,
/* FP:dfa.rs-0027 */     ref_transitions: Map<Reference<R, T>, State>,
/* FP:dfa.rs-0028 */ }
/* FP:dfa.rs-0029 */ 
/* FP:dfa.rs-0030 */ impl<R, T> Default for Transitions<R, T>
/* FP:dfa.rs-0031 */ where
/* FP:dfa.rs-0032 */     R: Region,
/* FP:dfa.rs-0033 */     T: Type,
/* FP:dfa.rs-0034 */ {
/* FP:dfa.rs-0035 */     fn default() -> Self {
/* FP:dfa.rs-0036 */         Self { byte_transitions: EdgeSet::empty(), ref_transitions: Map::default() }
/* FP:dfa.rs-0037 */     }
/* FP:dfa.rs-0038 */ }
/* FP:dfa.rs-0039 */ 
/* FP:dfa.rs-0040 */ /// The states in a [`Dfa`] represent byte offsets.
/* FP:dfa.rs-0041 */ #[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
/* FP:dfa.rs-0042 */ pub(crate) struct State(pub(crate) u32);
/* FP:dfa.rs-0043 */ 
/* FP:dfa.rs-0044 */ impl State {
/* FP:dfa.rs-0045 */     pub(crate) fn new() -> Self {
/* FP:dfa.rs-0046 */         static COUNTER: AtomicU32 = AtomicU32::new(0);
/* FP:dfa.rs-0047 */         Self(COUNTER.fetch_add(1, Ordering::SeqCst))
/* FP:dfa.rs-0048 */     }
/* FP:dfa.rs-0049 */ }
/* FP:dfa.rs-0050 */ 
/* FP:dfa.rs-0051 */ impl fmt::Debug for State {
/* FP:dfa.rs-0052 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:dfa.rs-0053 */         write!(f, "S_{}", self.0)
/* FP:dfa.rs-0054 */     }
/* FP:dfa.rs-0055 */ }
/* FP:dfa.rs-0056 */ 
/* FP:dfa.rs-0057 */ impl<R, T> Dfa<R, T>
/* FP:dfa.rs-0058 */ where
/* FP:dfa.rs-0059 */     R: Region,
/* FP:dfa.rs-0060 */     T: Type,
/* FP:dfa.rs-0061 */ {
/* FP:dfa.rs-0062 */     #[cfg(test)]
/* FP:dfa.rs-0063 */     pub(crate) fn bool() -> Self {
/* FP:dfa.rs-0064 */         Self::from_transitions(|accept| Transitions {
/* FP:dfa.rs-0065 */             byte_transitions: EdgeSet::new(Byte::new(0x00..=0x01), accept),
/* FP:dfa.rs-0066 */             ref_transitions: Map::default(),
/* FP:dfa.rs-0067 */         })
/* FP:dfa.rs-0068 */     }
/* FP:dfa.rs-0069 */ 
/* FP:dfa.rs-0070 */     pub(crate) fn unit() -> Self {
/* FP:dfa.rs-0071 */         let transitions: Map<State, Transitions<R, T>> = Map::default();
/* FP:dfa.rs-0072 */         let start = State::new();
/* FP:dfa.rs-0073 */         let accept = start;
/* FP:dfa.rs-0074 */ 
/* FP:dfa.rs-0075 */         Self { transitions, start, accept }
/* FP:dfa.rs-0076 */     }
/* FP:dfa.rs-0077 */ 
/* FP:dfa.rs-0078 */     pub(crate) fn from_byte(byte: Byte) -> Self {
/* FP:dfa.rs-0079 */         Self::from_transitions(|accept| Transitions {
/* FP:dfa.rs-0080 */             byte_transitions: EdgeSet::new(byte, accept),
/* FP:dfa.rs-0081 */             ref_transitions: Map::default(),
/* FP:dfa.rs-0082 */         })
/* FP:dfa.rs-0083 */     }
/* FP:dfa.rs-0084 */ 
/* FP:dfa.rs-0085 */     pub(crate) fn from_ref(r: Reference<R, T>) -> Self {
/* FP:dfa.rs-0086 */         Self::from_transitions(|accept| Transitions {
/* FP:dfa.rs-0087 */             byte_transitions: EdgeSet::empty(),
/* FP:dfa.rs-0088 */             ref_transitions: [(r, accept)].into_iter().collect(),
/* FP:dfa.rs-0089 */         })
/* FP:dfa.rs-0090 */     }
/* FP:dfa.rs-0091 */ 
/* FP:dfa.rs-0092 */     fn from_transitions(f: impl FnOnce(State) -> Transitions<R, T>) -> Self {
/* FP:dfa.rs-0093 */         let start = State::new();
/* FP:dfa.rs-0094 */         let accept = State::new();
/* FP:dfa.rs-0095 */ 
/* FP:dfa.rs-0096 */         Self { transitions: [(start, f(accept))].into_iter().collect(), start, accept }
/* FP:dfa.rs-0097 */     }
/* FP:dfa.rs-0098 */ 
/* FP:dfa.rs-0099 */     pub(crate) fn from_tree(tree: Tree<!, R, T>) -> Result<Self, Uninhabited> {
/* FP:dfa.rs-0100 */         Ok(match tree {
/* FP:dfa.rs-0101 */             Tree::Byte(b) => Self::from_byte(b),
/* FP:dfa.rs-0102 */             Tree::Ref(r) => Self::from_ref(r),
/* FP:dfa.rs-0103 */             Tree::Alt(alts) => {
/* FP:dfa.rs-0104 */                 // Convert and filter the inhabited alternatives.
/* FP:dfa.rs-0105 */                 let mut alts = alts.into_iter().map(Self::from_tree).filter_map(Result::ok);
/* FP:dfa.rs-0106 */                 // If there are no alternatives, return `Uninhabited`.
/* FP:dfa.rs-0107 */                 let dfa = alts.next().ok_or(Uninhabited)?;
/* FP:dfa.rs-0108 */                 // Combine the remaining alternatives with `dfa`.
/* FP:dfa.rs-0109 */                 alts.fold(dfa, |dfa, alt| dfa.union(alt, State::new))
/* FP:dfa.rs-0110 */             }
/* FP:dfa.rs-0111 */             Tree::Seq(elts) => {
/* FP:dfa.rs-0112 */                 let mut dfa = Self::unit();
/* FP:dfa.rs-0113 */                 for elt in elts.into_iter().map(Self::from_tree) {
/* FP:dfa.rs-0114 */                     dfa = dfa.concat(elt?);
/* FP:dfa.rs-0115 */                 }
/* FP:dfa.rs-0116 */                 dfa
/* FP:dfa.rs-0117 */             }
/* FP:dfa.rs-0118 */         })
/* FP:dfa.rs-0119 */     }
/* FP:dfa.rs-0120 */ 
/* FP:dfa.rs-0121 */     /// Concatenate two `Dfa`s.
/* FP:dfa.rs-0122 */     pub(crate) fn concat(self, other: Self) -> Self {
/* FP:dfa.rs-0123 */         if self.start == self.accept {
/* FP:dfa.rs-0124 */             return other;
/* FP:dfa.rs-0125 */         } else if other.start == other.accept {
/* FP:dfa.rs-0126 */             return self;
/* FP:dfa.rs-0127 */         }
/* FP:dfa.rs-0128 */ 
/* FP:dfa.rs-0129 */         let start = self.start;
/* FP:dfa.rs-0130 */         let accept = other.accept;
/* FP:dfa.rs-0131 */ 
/* FP:dfa.rs-0132 */         let mut transitions: Map<State, Transitions<R, T>> = self.transitions;
/* FP:dfa.rs-0133 */ 
/* FP:dfa.rs-0134 */         for (source, transition) in other.transitions {
/* FP:dfa.rs-0135 */             let fix_state = |state| if state == other.start { self.accept } else { state };
/* FP:dfa.rs-0136 */             let byte_transitions = transition.byte_transitions.map_states(&fix_state);
/* FP:dfa.rs-0137 */             let ref_transitions = transition
/* FP:dfa.rs-0138 */                 .ref_transitions
/* FP:dfa.rs-0139 */                 .into_iter()
/* FP:dfa.rs-0140 */                 .map(|(r, state)| (r, fix_state(state)))
/* FP:dfa.rs-0141 */                 .collect();
/* FP:dfa.rs-0142 */ 
/* FP:dfa.rs-0143 */             let old = transitions
/* FP:dfa.rs-0144 */                 .insert(fix_state(source), Transitions { byte_transitions, ref_transitions });
/* FP:dfa.rs-0145 */             assert!(old.is_none());
/* FP:dfa.rs-0146 */         }
/* FP:dfa.rs-0147 */ 
/* FP:dfa.rs-0148 */         Self { transitions, start, accept }
/* FP:dfa.rs-0149 */     }
/* FP:dfa.rs-0150 */ 
/* FP:dfa.rs-0151 */     /// Compute the union of two `Dfa`s.
/* FP:dfa.rs-0152 */     pub(crate) fn union(self, other: Self, mut new_state: impl FnMut() -> State) -> Self {
/* FP:dfa.rs-0153 */         // We implement `union` by lazily initializing a set of states
/* FP:dfa.rs-0154 */         // corresponding to the product of states in `self` and `other`, and
/* FP:dfa.rs-0155 */         // then add transitions between these states that correspond to where
/* FP:dfa.rs-0156 */         // they exist between `self` and `other`.
/* FP:dfa.rs-0157 */ 
/* FP:dfa.rs-0158 */         let a = self;
/* FP:dfa.rs-0159 */         let b = other;
/* FP:dfa.rs-0160 */ 
/* FP:dfa.rs-0161 */         let accept = new_state();
/* FP:dfa.rs-0162 */ 
/* FP:dfa.rs-0163 */         let mut mapping: Map<(Option<State>, Option<State>), State> = Map::default();
/* FP:dfa.rs-0164 */ 
/* FP:dfa.rs-0165 */         let mut mapped = |(a_state, b_state)| {
/* FP:dfa.rs-0166 */             if Some(a.accept) == a_state || Some(b.accept) == b_state {
/* FP:dfa.rs-0167 */                 // If either `a_state` or `b_state` are accepting, map to a
/* FP:dfa.rs-0168 */                 // common `accept` state.
/* FP:dfa.rs-0169 */                 accept
/* FP:dfa.rs-0170 */             } else {
/* FP:dfa.rs-0171 */                 *mapping.entry((a_state, b_state)).or_insert_with(&mut new_state)
/* FP:dfa.rs-0172 */             }
/* FP:dfa.rs-0173 */         };
/* FP:dfa.rs-0174 */ 
/* FP:dfa.rs-0175 */         let start = mapped((Some(a.start), Some(b.start)));
/* FP:dfa.rs-0176 */         let mut transitions: Map<State, Transitions<R, T>> = Map::default();
/* FP:dfa.rs-0177 */         let empty_transitions = Transitions::default();
/* FP:dfa.rs-0178 */ 
/* FP:dfa.rs-0179 */         struct WorkQueue {
/* FP:dfa.rs-0180 */             queue: Vec<(Option<State>, Option<State>)>,
/* FP:dfa.rs-0181 */             // Track all entries ever enqueued to avoid duplicating work. This
/* FP:dfa.rs-0182 */             // gives us a guarantee that a given (a_state, b_state) pair will
/* FP:dfa.rs-0183 */             // only ever be visited once.
/* FP:dfa.rs-0184 */             enqueued: Set<(Option<State>, Option<State>)>,
/* FP:dfa.rs-0185 */         }
/* FP:dfa.rs-0186 */         impl WorkQueue {
/* FP:dfa.rs-0187 */             fn enqueue(&mut self, a_state: Option<State>, b_state: Option<State>) {
/* FP:dfa.rs-0188 */                 if self.enqueued.insert((a_state, b_state)) {
/* FP:dfa.rs-0189 */                     self.queue.push((a_state, b_state));
/* FP:dfa.rs-0190 */                 }
/* FP:dfa.rs-0191 */             }
/* FP:dfa.rs-0192 */         }
/* FP:dfa.rs-0193 */         let mut queue = WorkQueue { queue: Vec::new(), enqueued: Set::default() };
/* FP:dfa.rs-0194 */         queue.enqueue(Some(a.start), Some(b.start));
/* FP:dfa.rs-0195 */ 
/* FP:dfa.rs-0196 */         while let Some((a_src, b_src)) = queue.queue.pop() {
/* FP:dfa.rs-0197 */             let src = mapped((a_src, b_src));
/* FP:dfa.rs-0198 */             if src == accept {
/* FP:dfa.rs-0199 */                 // While it's possible to have a DFA whose accept state has
/* FP:dfa.rs-0200 */                 // out-edges, these do not affect the semantics of the DFA, and
/* FP:dfa.rs-0201 */                 // so there's no point in processing them. Continuing here also
/* FP:dfa.rs-0202 */                 // has the advantage of guaranteeing that we only ever process a
/* FP:dfa.rs-0203 */                 // given node in the output DFA once. In particular, with the
/* FP:dfa.rs-0204 */                 // exception of the accept state, we ensure that we only push a
/* FP:dfa.rs-0205 */                 // given node to the `queue` once. This allows the following
/* FP:dfa.rs-0206 */                 // code to assume that we're processing a node we've never
/* FP:dfa.rs-0207 */                 // processed before, which means we never need to merge two edge
/* FP:dfa.rs-0208 */                 // sets - we only ever need to construct a new edge set from
/* FP:dfa.rs-0209 */                 // whole cloth.
/* FP:dfa.rs-0210 */                 continue;
/* FP:dfa.rs-0211 */             }
/* FP:dfa.rs-0212 */ 
/* FP:dfa.rs-0213 */             let a_transitions =
/* FP:dfa.rs-0214 */                 a_src.and_then(|a_src| a.transitions.get(&a_src)).unwrap_or(&empty_transitions);
/* FP:dfa.rs-0215 */             let b_transitions =
/* FP:dfa.rs-0216 */                 b_src.and_then(|b_src| b.transitions.get(&b_src)).unwrap_or(&empty_transitions);
/* FP:dfa.rs-0217 */ 
/* FP:dfa.rs-0218 */             let byte_transitions = a_transitions.byte_transitions.union(
/* FP:dfa.rs-0219 */                 &b_transitions.byte_transitions,
/* FP:dfa.rs-0220 */                 |a_dst, b_dst| {
/* FP:dfa.rs-0221 */                     assert!(a_dst.is_some() || b_dst.is_some());
/* FP:dfa.rs-0222 */ 
/* FP:dfa.rs-0223 */                     queue.enqueue(a_dst, b_dst);
/* FP:dfa.rs-0224 */                     mapped((a_dst, b_dst))
/* FP:dfa.rs-0225 */                 },
/* FP:dfa.rs-0226 */             );
/* FP:dfa.rs-0227 */ 
/* FP:dfa.rs-0228 */             let ref_transitions =
/* FP:dfa.rs-0229 */                 a_transitions.ref_transitions.keys().chain(b_transitions.ref_transitions.keys());
/* FP:dfa.rs-0230 */ 
/* FP:dfa.rs-0231 */             let ref_transitions = ref_transitions
/* FP:dfa.rs-0232 */                 .map(|ref_transition| {
/* FP:dfa.rs-0233 */                     let a_dst = a_transitions.ref_transitions.get(ref_transition).copied();
/* FP:dfa.rs-0234 */                     let b_dst = b_transitions.ref_transitions.get(ref_transition).copied();
/* FP:dfa.rs-0235 */ 
/* FP:dfa.rs-0236 */                     assert!(a_dst.is_some() || b_dst.is_some());
/* FP:dfa.rs-0237 */ 
/* FP:dfa.rs-0238 */                     queue.enqueue(a_dst, b_dst);
/* FP:dfa.rs-0239 */                     (*ref_transition, mapped((a_dst, b_dst)))
/* FP:dfa.rs-0240 */                 })
/* FP:dfa.rs-0241 */                 .collect();
/* FP:dfa.rs-0242 */ 
/* FP:dfa.rs-0243 */             let old = transitions.insert(src, Transitions { byte_transitions, ref_transitions });
/* FP:dfa.rs-0244 */             // See `if src == accept { ... }` above. The comment there explains
/* FP:dfa.rs-0245 */             // why this assert is valid.
/* FP:dfa.rs-0246 */             assert_eq!(old, None);
/* FP:dfa.rs-0247 */         }
/* FP:dfa.rs-0248 */ 
/* FP:dfa.rs-0249 */         Self { transitions, start, accept }
/* FP:dfa.rs-0250 */     }
/* FP:dfa.rs-0251 */ 
/* FP:dfa.rs-0252 */     pub(crate) fn get_uninit_edge_dst(&self, state: State) -> Option<State> {
/* FP:dfa.rs-0253 */         let transitions = self.transitions.get(&state)?;
/* FP:dfa.rs-0254 */         transitions.byte_transitions.get_uninit_edge_dst()
/* FP:dfa.rs-0255 */     }
/* FP:dfa.rs-0256 */ 
/* FP:dfa.rs-0257 */     pub(crate) fn bytes_from(&self, start: State) -> impl Iterator<Item = (Byte, State)> {
/* FP:dfa.rs-0258 */         self.transitions
/* FP:dfa.rs-0259 */             .get(&start)
/* FP:dfa.rs-0260 */             .into_iter()
/* FP:dfa.rs-0261 */             .flat_map(|transitions| transitions.byte_transitions.iter())
/* FP:dfa.rs-0262 */     }
/* FP:dfa.rs-0263 */ 
/* FP:dfa.rs-0264 */     pub(crate) fn refs_from(&self, start: State) -> impl Iterator<Item = (Reference<R, T>, State)> {
/* FP:dfa.rs-0265 */         self.transitions
/* FP:dfa.rs-0266 */             .get(&start)
/* FP:dfa.rs-0267 */             .into_iter()
/* FP:dfa.rs-0268 */             .flat_map(|transitions| transitions.ref_transitions.iter())
/* FP:dfa.rs-0269 */             .map(|(r, s)| (*r, *s))
/* FP:dfa.rs-0270 */     }
/* FP:dfa.rs-0271 */ 
/* FP:dfa.rs-0272 */     #[cfg(test)]
/* FP:dfa.rs-0273 */     pub(crate) fn from_edges<B: Clone + Into<Byte>>(
/* FP:dfa.rs-0274 */         start: u32,
/* FP:dfa.rs-0275 */         accept: u32,
/* FP:dfa.rs-0276 */         edges: &[(u32, B, u32)],
/* FP:dfa.rs-0277 */     ) -> Self {
/* FP:dfa.rs-0278 */         let start = State(start);
/* FP:dfa.rs-0279 */         let accept = State(accept);
/* FP:dfa.rs-0280 */         let mut transitions: Map<State, Vec<(Byte, State)>> = Map::default();
/* FP:dfa.rs-0281 */ 
/* FP:dfa.rs-0282 */         for &(src, ref edge, dst) in edges.iter() {
/* FP:dfa.rs-0283 */             transitions.entry(State(src)).or_default().push((edge.clone().into(), State(dst)));
/* FP:dfa.rs-0284 */         }
/* FP:dfa.rs-0285 */ 
/* FP:dfa.rs-0286 */         let transitions = transitions
/* FP:dfa.rs-0287 */             .into_iter()
/* FP:dfa.rs-0288 */             .map(|(src, edges)| {
/* FP:dfa.rs-0289 */                 (
/* FP:dfa.rs-0290 */                     src,
/* FP:dfa.rs-0291 */                     Transitions {
/* FP:dfa.rs-0292 */                         byte_transitions: EdgeSet::from_edges(edges),
/* FP:dfa.rs-0293 */                         ref_transitions: Map::default(),
/* FP:dfa.rs-0294 */                     },
/* FP:dfa.rs-0295 */                 )
/* FP:dfa.rs-0296 */             })
/* FP:dfa.rs-0297 */             .collect();
/* FP:dfa.rs-0298 */ 
/* FP:dfa.rs-0299 */         Self { start, accept, transitions }
/* FP:dfa.rs-0300 */     }
/* FP:dfa.rs-0301 */ }
/* FP:dfa.rs-0302 */ 
/* FP:dfa.rs-0303 */ /// Serialize the DFA using the Graphviz DOT format.
/* FP:dfa.rs-0304 */ impl<R, T> fmt::Debug for Dfa<R, T>
/* FP:dfa.rs-0305 */ where
/* FP:dfa.rs-0306 */     R: Region,
/* FP:dfa.rs-0307 */     T: Type,
/* FP:dfa.rs-0308 */ {
/* FP:dfa.rs-0309 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:dfa.rs-0310 */         writeln!(f, "digraph {{")?;
/* FP:dfa.rs-0311 */         writeln!(f, "    {:?} [shape = doublecircle]", self.start)?;
/* FP:dfa.rs-0312 */         writeln!(f, "    {:?} [shape = doublecircle]", self.accept)?;
/* FP:dfa.rs-0313 */ 
/* FP:dfa.rs-0314 */         for (src, transitions) in self.transitions.iter() {
/* FP:dfa.rs-0315 */             for (t, dst) in transitions.byte_transitions.iter() {
/* FP:dfa.rs-0316 */                 writeln!(f, "    {src:?} -> {dst:?} [label=\"{t:?}\"]")?;
/* FP:dfa.rs-0317 */             }
/* FP:dfa.rs-0318 */ 
/* FP:dfa.rs-0319 */             for (t, dst) in transitions.ref_transitions.iter() {
/* FP:dfa.rs-0320 */                 writeln!(f, "    {src:?} -> {dst:?} [label=\"{t:?}\"]")?;
/* FP:dfa.rs-0321 */             }
/* FP:dfa.rs-0322 */         }
/* FP:dfa.rs-0323 */ 
/* FP:dfa.rs-0324 */         writeln!(f, "}}")
/* FP:dfa.rs-0325 */     }
/* FP:dfa.rs-0326 */ }
/* FP:dfa.rs-0327 */ 
/* FP:dfa.rs-0328 */ use edge_set::EdgeSet;
/* FP:dfa.rs-0329 */ mod edge_set {
/* FP:dfa.rs-0330 */     use smallvec::SmallVec;
/* FP:dfa.rs-0331 */ 
/* FP:dfa.rs-0332 */     use super::*;
/* FP:dfa.rs-0333 */ 
/* FP:dfa.rs-0334 */     /// The set of outbound byte edges associated with a DFA node.
/* FP:dfa.rs-0335 */     #[derive(Eq, PartialEq, Clone, Debug)]
/* FP:dfa.rs-0336 */     pub(super) struct EdgeSet<S = State> {
/* FP:dfa.rs-0337 */         // A sequence of byte edges with contiguous byte values and a common
/* FP:dfa.rs-0338 */         // destination is stored as a single run.
/* FP:dfa.rs-0339 */         //
/* FP:dfa.rs-0340 */         // Runs are non-empty, non-overlapping, and stored in ascending order.
/* FP:dfa.rs-0341 */         runs: SmallVec<[(Byte, S); 1]>,
/* FP:dfa.rs-0342 */     }
/* FP:dfa.rs-0343 */ 
/* FP:dfa.rs-0344 */     impl<S> EdgeSet<S> {
/* FP:dfa.rs-0345 */         pub(crate) fn new(range: Byte, dst: S) -> Self {
/* FP:dfa.rs-0346 */             let mut this = Self { runs: SmallVec::new() };
/* FP:dfa.rs-0347 */             if !range.is_empty() {
/* FP:dfa.rs-0348 */                 this.runs.push((range, dst));
/* FP:dfa.rs-0349 */             }
/* FP:dfa.rs-0350 */             this
/* FP:dfa.rs-0351 */         }
/* FP:dfa.rs-0352 */ 
/* FP:dfa.rs-0353 */         pub(crate) fn empty() -> Self {
/* FP:dfa.rs-0354 */             Self { runs: SmallVec::new() }
/* FP:dfa.rs-0355 */         }
/* FP:dfa.rs-0356 */ 
/* FP:dfa.rs-0357 */         #[cfg(test)]
/* FP:dfa.rs-0358 */         pub(crate) fn from_edges(mut edges: Vec<(Byte, S)>) -> Self
/* FP:dfa.rs-0359 */         where
/* FP:dfa.rs-0360 */             S: Ord,
/* FP:dfa.rs-0361 */         {
/* FP:dfa.rs-0362 */             edges.sort();
/* FP:dfa.rs-0363 */             Self { runs: edges.into() }
/* FP:dfa.rs-0364 */         }
/* FP:dfa.rs-0365 */ 
/* FP:dfa.rs-0366 */         pub(crate) fn iter(&self) -> impl Iterator<Item = (Byte, S)>
/* FP:dfa.rs-0367 */         where
/* FP:dfa.rs-0368 */             S: Copy,
/* FP:dfa.rs-0369 */         {
/* FP:dfa.rs-0370 */             self.runs.iter().copied()
/* FP:dfa.rs-0371 */         }
/* FP:dfa.rs-0372 */ 
/* FP:dfa.rs-0373 */         pub(crate) fn get_uninit_edge_dst(&self) -> Option<S>
/* FP:dfa.rs-0374 */         where
/* FP:dfa.rs-0375 */             S: Copy,
/* FP:dfa.rs-0376 */         {
/* FP:dfa.rs-0377 */             // Uninit is ordered last.
/* FP:dfa.rs-0378 */             let &(range, dst) = self.runs.last()?;
/* FP:dfa.rs-0379 */             if range.contains_uninit() { Some(dst) } else { None }
/* FP:dfa.rs-0380 */         }
/* FP:dfa.rs-0381 */ 
/* FP:dfa.rs-0382 */         pub(crate) fn map_states<SS>(self, mut f: impl FnMut(S) -> SS) -> EdgeSet<SS> {
/* FP:dfa.rs-0383 */             EdgeSet {
/* FP:dfa.rs-0384 */                 // NOTE: It appears as through `<Vec<_> as
/* FP:dfa.rs-0385 */                 // IntoIterator>::IntoIter` and `std::iter::Map` both implement
/* FP:dfa.rs-0386 */                 // `TrustedLen`, which in turn means that this `.collect()`
/* FP:dfa.rs-0387 */                 // allocates the correct number of elements once up-front [1].
/* FP:dfa.rs-0388 */                 //
/* FP:dfa.rs-0389 */                 // [1] https://doc.rust-lang.org/1.85.0/src/alloc/vec/spec_from_iter_nested.rs.html#47
/* FP:dfa.rs-0390 */                 runs: self.runs.into_iter().map(|(b, s)| (b, f(s))).collect(),
/* FP:dfa.rs-0391 */             }
/* FP:dfa.rs-0392 */         }
/* FP:dfa.rs-0393 */ 
/* FP:dfa.rs-0394 */         /// Unions two edge sets together.
/* FP:dfa.rs-0395 */         ///
/* FP:dfa.rs-0396 */         /// If `u = a.union(b)`, then for each byte value, `u` will have an edge
/* FP:dfa.rs-0397 */         /// with that byte value and with the destination `join(Some(_), None)`,
/* FP:dfa.rs-0398 */         /// `join(None, Some(_))`, or `join(Some(_), Some(_))` depending on whether `a`,
/* FP:dfa.rs-0399 */         /// `b`, or both have an edge with that byte value.
/* FP:dfa.rs-0400 */         ///
/* FP:dfa.rs-0401 */         /// If neither `a` nor `b` have an edge with a particular byte value,
/* FP:dfa.rs-0402 */         /// then no edge with that value will be present in `u`.
/* FP:dfa.rs-0403 */         pub(crate) fn union(
/* FP:dfa.rs-0404 */             &self,
/* FP:dfa.rs-0405 */             other: &Self,
/* FP:dfa.rs-0406 */             mut join: impl FnMut(Option<S>, Option<S>) -> S,
/* FP:dfa.rs-0407 */         ) -> EdgeSet<S>
/* FP:dfa.rs-0408 */         where
/* FP:dfa.rs-0409 */             S: Copy + Eq,
/* FP:dfa.rs-0410 */         {
/* FP:dfa.rs-0411 */             let mut runs: SmallVec<[(Byte, S); 1]> = SmallVec::new();
/* FP:dfa.rs-0412 */             let xs = self.runs.iter().copied();
/* FP:dfa.rs-0413 */             let ys = other.runs.iter().copied();
/* FP:dfa.rs-0414 */             for (range, (x, y)) in union(xs, ys) {
/* FP:dfa.rs-0415 */                 let state = join(x, y);
/* FP:dfa.rs-0416 */                 match runs.last_mut() {
/* FP:dfa.rs-0417 */                     // Merge contiguous runs with a common destination.
/* FP:dfa.rs-0418 */                     Some(&mut (ref mut last_range, ref mut last_state))
/* FP:dfa.rs-0419 */                         if last_range.end == range.start && *last_state == state =>
/* FP:dfa.rs-0420 */                     {
/* FP:dfa.rs-0421 */                         last_range.end = range.end
/* FP:dfa.rs-0422 */                     }
/* FP:dfa.rs-0423 */                     _ => runs.push((range, state)),
/* FP:dfa.rs-0424 */                 }
/* FP:dfa.rs-0425 */             }
/* FP:dfa.rs-0426 */             EdgeSet { runs }
/* FP:dfa.rs-0427 */         }
/* FP:dfa.rs-0428 */     }
/* FP:dfa.rs-0429 */ }
/* FP:dfa.rs-0430 */ 
/* FP:dfa.rs-0431 */ /// Merges two sorted sequences into one sorted sequence.
/* FP:dfa.rs-0432 */ pub(crate) fn union<S: Copy, X: Iterator<Item = (Byte, S)>, Y: Iterator<Item = (Byte, S)>>(
/* FP:dfa.rs-0433 */     xs: X,
/* FP:dfa.rs-0434 */     ys: Y,
/* FP:dfa.rs-0435 */ ) -> UnionIter<X, Y> {
/* FP:dfa.rs-0436 */     UnionIter { xs: xs.peekable(), ys: ys.peekable() }
/* FP:dfa.rs-0437 */ }
/* FP:dfa.rs-0438 */ 
/* FP:dfa.rs-0439 */ pub(crate) struct UnionIter<X: Iterator, Y: Iterator> {
/* FP:dfa.rs-0440 */     xs: Peekable<X>,
/* FP:dfa.rs-0441 */     ys: Peekable<Y>,
/* FP:dfa.rs-0442 */ }
/* FP:dfa.rs-0443 */ 
/* FP:dfa.rs-0444 */ // FIXME(jswrenn) we'd likely benefit from specializing try_fold here.
/* FP:dfa.rs-0445 */ impl<S: Copy, X: Iterator<Item = (Byte, S)>, Y: Iterator<Item = (Byte, S)>> Iterator
/* FP:dfa.rs-0446 */     for UnionIter<X, Y>
/* FP:dfa.rs-0447 */ {
/* FP:dfa.rs-0448 */     type Item = (Byte, (Option<S>, Option<S>));
/* FP:dfa.rs-0449 */ 
/* FP:dfa.rs-0450 */     fn next(&mut self) -> Option<Self::Item> {
/* FP:dfa.rs-0451 */         use std::cmp::{self, Ordering};
/* FP:dfa.rs-0452 */ 
/* FP:dfa.rs-0453 */         let ret;
/* FP:dfa.rs-0454 */         match (self.xs.peek_mut(), self.ys.peek_mut()) {
/* FP:dfa.rs-0455 */             (None, None) => {
/* FP:dfa.rs-0456 */                 ret = None;
/* FP:dfa.rs-0457 */             }
/* FP:dfa.rs-0458 */             (Some(x), None) => {
/* FP:dfa.rs-0459 */                 ret = Some((x.0, (Some(x.1), None)));
/* FP:dfa.rs-0460 */                 self.xs.next();
/* FP:dfa.rs-0461 */             }
/* FP:dfa.rs-0462 */             (None, Some(y)) => {
/* FP:dfa.rs-0463 */                 ret = Some((y.0, (None, Some(y.1))));
/* FP:dfa.rs-0464 */                 self.ys.next();
/* FP:dfa.rs-0465 */             }
/* FP:dfa.rs-0466 */             (Some(x), Some(y)) => {
/* FP:dfa.rs-0467 */                 let start;
/* FP:dfa.rs-0468 */                 let end;
/* FP:dfa.rs-0469 */                 let dst;
/* FP:dfa.rs-0470 */                 match x.0.start.cmp(&y.0.start) {
/* FP:dfa.rs-0471 */                     Ordering::Less => {
/* FP:dfa.rs-0472 */                         start = x.0.start;
/* FP:dfa.rs-0473 */                         end = cmp::min(x.0.end, y.0.start);
/* FP:dfa.rs-0474 */                         dst = (Some(x.1), None);
/* FP:dfa.rs-0475 */                     }
/* FP:dfa.rs-0476 */                     Ordering::Greater => {
/* FP:dfa.rs-0477 */                         start = y.0.start;
/* FP:dfa.rs-0478 */                         end = cmp::min(x.0.start, y.0.end);
/* FP:dfa.rs-0479 */                         dst = (None, Some(y.1));
/* FP:dfa.rs-0480 */                     }
/* FP:dfa.rs-0481 */                     Ordering::Equal => {
/* FP:dfa.rs-0482 */                         start = x.0.start;
/* FP:dfa.rs-0483 */                         end = cmp::min(x.0.end, y.0.end);
/* FP:dfa.rs-0484 */                         dst = (Some(x.1), Some(y.1));
/* FP:dfa.rs-0485 */                     }
/* FP:dfa.rs-0486 */                 }
/* FP:dfa.rs-0487 */                 ret = Some((Byte { start, end }, dst));
/* FP:dfa.rs-0488 */                 if start == x.0.start {
/* FP:dfa.rs-0489 */                     x.0.start = end;
/* FP:dfa.rs-0490 */                 }
/* FP:dfa.rs-0491 */                 if start == y.0.start {
/* FP:dfa.rs-0492 */                     y.0.start = end;
/* FP:dfa.rs-0493 */                 }
/* FP:dfa.rs-0494 */                 if x.0.is_empty() {
/* FP:dfa.rs-0495 */                     self.xs.next();
/* FP:dfa.rs-0496 */                 }
/* FP:dfa.rs-0497 */                 if y.0.is_empty() {
/* FP:dfa.rs-0498 */                     self.ys.next();
/* FP:dfa.rs-0499 */                 }
/* FP:dfa.rs-0500 */             }
/* FP:dfa.rs-0501 */         }
/* FP:dfa.rs-0502 */         ret
/* FP:dfa.rs-0503 */     }
/* FP:dfa.rs-0504 */ }