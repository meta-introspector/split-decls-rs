// Generated macro for macro_85 (macro)
macro_rules! Depcrate_visitmacro_85 {
() => {
// Module: crate::visit
// Provides: {"macro_85"}
// Dependencies: {}
trait_template ! { # [doc = " Base graph trait: defines the associated node identifier and"] # [doc = " edge identifier types."] pub trait GraphBase { @ escape [type NodeId] @ escape [type EdgeId] @ section nodelegate # [doc = " edge identifier"] type EdgeId : Copy + PartialEq ; # [doc = " node identifier"] type NodeId : Copy + PartialEq ; } }
};
}
