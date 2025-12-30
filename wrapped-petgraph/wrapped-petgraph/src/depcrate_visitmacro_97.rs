// Generated macro for macro_97 (macro)
macro_rules! Depcrate_visitmacro_97 {
() => {
// Module: crate::visit
// Provides: {"macro_97"}
// Dependencies: {}
trait_template ! { # [doc = " Access to the sequence of the graph’s `NodeId`s."] pub trait IntoNodeIdentifiers : GraphRef { @ section type type NodeIdentifiers : Iterator < Item = Self :: NodeId >; @ section self fn node_identifiers (self) -> Self :: NodeIdentifiers ; } }
};
}
