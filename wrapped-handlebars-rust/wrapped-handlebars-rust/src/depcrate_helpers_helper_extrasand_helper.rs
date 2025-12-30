// Generated macro for AND_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasAND_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"AND_HELPER"}
// Dependencies: {}
pub (crate) static AND_HELPER : ManyBoolHelper = ManyBoolHelper { name : "and" , op : | params | params . iter () . all (| p | p . value () . is_truthy (false)) , } ;
};
}
