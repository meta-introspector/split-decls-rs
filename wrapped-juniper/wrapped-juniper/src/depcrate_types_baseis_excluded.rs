// Generated macro for is_excluded (function)
macro_rules! Depcrate_types_baseis_excluded {
() => {
// Module: crate::types::base
// Provides: {"is_excluded"}
// Dependencies: {}
pub (super) fn is_excluded < S > (directives : & Option < Vec < Spanning < Directive < S > > > > , vars : & Variables < S > ,) -> bool where S : ScalarValue , { if let Some (directives) = directives { for Spanning { item : directive , .. } in directives { let condition : bool = directive . arguments . iter () . flat_map (| m | m . item . get ("if")) . filter_map (| v | v . item . clone () . into_const (vars) ? . convert () . ok ()) . next () . unwrap () ; if (directive . name . item == "skip" && condition) || (directive . name . item == "include" && ! condition) { return true ; } } } false }
};
}
