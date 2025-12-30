// Generated macro for macro_120 (macro)
macro_rules! Depcrate_visitmacro_120 {
() => {
// Module: crate::visit
// Provides: {"macro_120"}
// Dependencies: {}
trait_template ! { # [doc = " The graph’s `NodeId`s map to indices, in a range without holes."] # [doc = ""] # [doc = " The graph's node identifiers correspond to exactly the indices"] # [doc = " `0..self.node_bound()`."] pub trait NodeCompactIndexable : NodeIndexable + NodeCount { } }
};
}
