// Generated macro for _is_used (function)
macro_rules! Depcrate_utils_is_used {
() => {
// Module: crate::utils
// Provides: {"_is_used"}
// Dependencies: {}
fn _is_used (visited : & mut HashSet < Ident > , id : & Ident , references : & HashMap < Ident , HashSet < Ident > > , ends : & HashSet < Ident > ,) -> bool { if visited . contains (id) { return false ; } visited . insert (id . clone ()) ; if ends . contains (id) { return true ; } if references . contains_key (id) { for referred in references . get (id) . unwrap () { if _is_used (visited , referred , references , ends) { return true ; } } } false }
};
}
