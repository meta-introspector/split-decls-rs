// Generated macro for ParseResultBase (trait)
macro_rules! Depcrate_parse_resultParseResultBase {
() => {
// Module: crate::parse_result
// Provides: {"ParseResultBase"}
// Dependencies: {}
pub trait ParseResultBase < T > { fn handle_failure (& mut self , tracker : & mut DynMTrackerTrait ! ()) ; fn is_ok (& self) -> bool ; fn unwrap (self) -> Option < T > ; }
};
}
