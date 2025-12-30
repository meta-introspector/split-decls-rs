// Generated macro for path_ty_layout (function)
macro_rules! Depcrate_helperspath_ty_layout {
() => {
// Module: crate::helpers
// Provides: {"path_ty_layout"}
// Dependencies: {}
# [doc = " Gets the layout of a type at a path."] # [track_caller] pub fn path_ty_layout < 'tcx > (cx : & impl LayoutOf < 'tcx > , path : & [& str]) -> TyAndLayout < 'tcx > { let ty = resolve_path (cx . tcx () , path , Namespace :: TypeNS) . ty (cx . tcx () , cx . typing_env ()) ; cx . layout_of (ty) . to_result () . ok () . unwrap () }
};
}
