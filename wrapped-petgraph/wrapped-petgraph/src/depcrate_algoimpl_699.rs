// Generated macro for impl_699 (impl)
macro_rules! Depcrate_algoimpl_699 {
() => {
// Module: crate::algo
// Provides: {"impl_699"}
// Dependencies: {}
impl < N , VM > DfsSpace < N , VM > where N : Copy + PartialEq , VM : VisitMap < N > , { pub fn new < G > (g : G) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { DfsSpace { dfs : Dfs :: empty (g) } } }
};
}
