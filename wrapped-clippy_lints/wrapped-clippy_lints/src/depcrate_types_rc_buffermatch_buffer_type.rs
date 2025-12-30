// Generated macro for match_buffer_type (function)
macro_rules! Depcrate_types_rc_buffermatch_buffer_type {
() => {
// Module: crate::types::rc_buffer
// Provides: {"match_buffer_type"}
// Dependencies: {}
fn match_buffer_type (cx : & LateContext < '_ > , ty : & Ty < '_ > , applicability : & mut Applicability ,) -> Option < Cow < 'static , str > > { let id = ty . basic_res () . opt_def_id () ? ; let path = match cx . tcx . get_diagnostic_name (id) { Some (sym :: OsString) => "std::ffi::OsStr" . into () , Some (sym :: PathBuf) => "std::path::Path" . into () , Some (sym :: Vec) => { let TyKind :: Path (vec_qpath) = & ty . kind else { return None ; } ; let vec_generic_ty = qpath_generic_tys (vec_qpath) . next () ? ; let snippet = snippet_with_applicability (cx , vec_generic_ty . span , "_" , applicability) ; format ! ("[{snippet}]") . into () } , _ if Some (id) == cx . tcx . lang_items () . string () => "str" . into () , _ => return None , } ; Some (path) }
};
}
