// Generated macro for qpath_generic_tys (function)
macro_rules! Depcrateqpath_generic_tys {
() => {
// Module: crate
// Provides: {"qpath_generic_tys"}
// Dependencies: {}
pub fn qpath_generic_tys < 'tcx > (qpath : & QPath < 'tcx >) -> impl Iterator < Item = & 'tcx hir :: Ty < 'tcx > > { last_path_segment (qpath) . args . map_or (& [] [..] , | a | a . args) . iter () . filter_map (| a | match a { GenericArg :: Type (ty) => Some (ty . as_unambig_ty ()) , _ => None , }) }
};
}
