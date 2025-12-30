// Generated macro for update_res (function)
macro_rules! Depcrate_ty_type_certaintyupdate_res {
() => {
// Module: crate::ty::type_certainty
// Provides: {"update_res"}
// Dependencies: {}
# [doc = " For at least some `QPath::TypeRelative`, the path segment's `res` can be `Res::Err`."] # [doc = " `update_res` tries to fix the resolution when `parent_certainty` is `Certain(Some(..))`."] fn update_res (cx : & LateContext < '_ > , parent_certainty : Certainty , path_segment : & PathSegment < '_ > , resolves_to_type : bool ,) -> Option < Res > { if path_segment . res == Res :: Err && let Some (def_id) = parent_certainty . to_def_id () { let mut def_path = cx . get_def_path (def_id) ; def_path . push (path_segment . ident . name) ; let ns = if resolves_to_type { PathNS :: Type } else { PathNS :: Value } ; if let & [id] = lookup_path (cx . tcx , ns , & def_path) . as_slice () { return Some (Res :: Def (cx . tcx . def_kind (id) , id)) ; } } None }
};
}
