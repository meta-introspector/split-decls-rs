// Generated macro for macro_110 (macro)
macro_rules! Depcrate_visitmacro_110 {
() => {
// Module: crate::visit
// Provides: {"macro_110"}
// Dependencies: {}
trait_template ! { # [doc = " Access to the sequence of the graph’s edges"] pub trait IntoEdgeReferences : Data + GraphRef { @ section type type EdgeRef : EdgeRef < NodeId = Self :: NodeId , EdgeId = Self :: EdgeId , Weight = Self :: EdgeWeight >; type EdgeReferences : Iterator < Item = Self :: EdgeRef >; @ section self fn edge_references (self) -> Self :: EdgeReferences ; } }
};
}
