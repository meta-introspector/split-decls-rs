// Generated macro for is_eq (function)
macro_rules! Depcrate_walk_classifyis_eq {
() => {
// Module: crate::walk::classify
// Provides: {"is_eq"}
// Dependencies: {}
fn is_eq (lhs : & BStr , rhs : impl AsRef < BStr > , ignore_case : bool) -> bool { if ignore_case { lhs . eq_ignore_ascii_case (rhs . as_ref () . as_ref ()) } else { lhs == rhs . as_ref () } }
};
}
