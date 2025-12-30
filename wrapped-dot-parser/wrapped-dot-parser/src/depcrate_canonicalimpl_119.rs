// Generated macro for impl_119 (impl)
macro_rules! Depcrate_canonicalimpl_119 {
() => {
// Module: crate::canonical
// Provides: {"impl_119"}
// Dependencies: {}
impl < A , I > From < I > for NodeSet < A > where I : IntoIterator < Item = NodeStmt < A > > , { fn from (nodes : I) -> Self { let set : HashMap < _ , _ > = nodes . into_iter () . map (| node | (node . node . id . to_string () , node . into ())) . collect () ; NodeSet { set } } }
};
}
