// Generated macro for is_candidate_for_elision (function)
macro_rules! Depcrate_lifetimesis_candidate_for_elision {
() => {
// Module: crate::lifetimes
// Provides: {"is_candidate_for_elision"}
// Dependencies: {}
# [doc = " Check if `fd` supports function elision with an anonymous (or elided) lifetime,"] # [doc = " and has a lifetime somewhere in its output type."] fn is_candidate_for_elision (fd : & FnDecl < '_ >) -> bool { struct V ; impl Visitor < '_ > for V { type Result = ControlFlow < bool > ; fn visit_lifetime (& mut self , lifetime : & Lifetime) -> Self :: Result { ControlFlow :: Break (lifetime . is_elided () || lifetime . is_anonymous ()) } } if fd . lifetime_elision_allowed && let Return (ret_ty) = fd . output && walk_unambig_ty (& mut V , ret_ty) . is_break () { fd . inputs . iter () . find_map (| ty | walk_unambig_ty (& mut V , ty) . break_value ()) . unwrap () } else { false } }
};
}
