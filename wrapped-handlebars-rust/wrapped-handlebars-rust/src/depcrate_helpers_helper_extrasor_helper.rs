// Generated macro for OR_HELPER (static)
macro_rules! Depcrate_helpers_helper_extrasOR_HELPER {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"OR_HELPER"}
// Dependencies: {}
pub (crate) static OR_HELPER : ManyBoolHelper = ManyBoolHelper { name : "or" , op : | params | params . iter () . any (| p | p . value () . is_truthy (false)) , } ;
};
}
