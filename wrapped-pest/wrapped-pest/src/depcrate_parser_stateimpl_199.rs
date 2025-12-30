// Generated macro for impl_199 (impl)
macro_rules! Depcrate_parser_stateimpl_199 {
() => {
// Module: crate::parser_state
// Provides: {"impl_199"}
// Dependencies: {}
impl < R > ParseAttempt < R > { pub fn get_rule (& self) -> Option < & R > { match self { ParseAttempt :: Rule (r) => Some (r) , ParseAttempt :: Token => None , } } }
};
}
