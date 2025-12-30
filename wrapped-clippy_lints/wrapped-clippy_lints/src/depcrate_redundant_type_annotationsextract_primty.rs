// Generated macro for extract_primty (function)
macro_rules! Depcrate_redundant_type_annotationsextract_primty {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"extract_primty"}
// Dependencies: {}
fn extract_primty (ty_kind : & hir :: TyKind < '_ >) -> Option < hir :: PrimTy > { if let hir :: TyKind :: Path (ty_path) = ty_kind && let hir :: QPath :: Resolved (_ , resolved_path_ty) = ty_path && let hir :: def :: Res :: PrimTy (primty) = resolved_path_ty . res { Some (primty) } else { None } }
};
}
