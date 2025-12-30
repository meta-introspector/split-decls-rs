// Generated macro for impl_32 (impl)
macro_rules! Depcrate_testsimpl_32 {
() => {
// Module: crate::tests
// Provides: {"impl_32"}
// Dependencies: {}
impl NodeLabels < & 'static str > { fn to_opt_strs (self) -> Vec < Option < & 'static str > > { match self { UnlabelledNodes (len) => vec ! [None ; len] , AllNodesLabelled (lbls) => lbls . into_iter () . map (Some) . collect () , SomeNodesLabelled (lbls) => lbls , } } fn len (& self) -> usize { match self { & UnlabelledNodes (len) => len , & AllNodesLabelled (ref lbls) => lbls . len () , & SomeNodesLabelled (ref lbls) => lbls . len () , } } }
};
}
