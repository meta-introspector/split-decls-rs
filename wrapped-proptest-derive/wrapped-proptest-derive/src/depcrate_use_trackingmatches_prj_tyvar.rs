// Generated macro for matches_prj_tyvar (function)
macro_rules! Depcrate_use_trackingmatches_prj_tyvar {
() => {
// Module: crate::use_tracking
// Provides: {"matches_prj_tyvar"}
// Dependencies: {}
fn matches_prj_tyvar (ut : & mut UseTracker , tpath : & syn :: TypePath) -> bool { let path = & tpath . path ; let segs = & path . segments ; if let Some (qself) = & tpath . qself { if let Some (sub_tp) = extract_path (& qself . ty) { return sub_tp . qself . is_none () && util :: match_singleton (segs . iter () . skip (qself . position)) . filter (| ps | ps . arguments . is_empty ()) . and_then (| _ | util :: extract_simple_path (& sub_tp . path)) . filter (| & ident | ut . has_tyvar (ident)) . is_some () || matches_prj_tyvar (ut , sub_tp) ; } false } else { return ! util :: path_is_global (path) && segs . len () == 2 && ut . has_tyvar (& segs [0] . ident) && segs [0] . arguments . is_empty () && segs [1] . arguments . is_empty () ; } }
};
}
