// Generated macro for GenericsArgsErrExtend (enum)
macro_rules! Depcrate_hir_ty_lowering_errorsGenericsArgsErrExtend {
() => {
// Module: crate::hir_ty_lowering::errors
// Provides: {"GenericsArgsErrExtend"}
// Dependencies: {}
# [doc = " Used for generics args error extend."] pub enum GenericsArgsErrExtend < 'tcx > { EnumVariant { qself : & 'tcx hir :: Ty < 'tcx > , assoc_segment : & 'tcx hir :: PathSegment < 'tcx > , adt_def : AdtDef < 'tcx > , } , OpaqueTy , PrimTy (hir :: PrimTy) , SelfTyAlias { def_id : DefId , span : Span , } , SelfTyParam (Span) , Param (DefId) , DefVariant (& 'tcx [hir :: PathSegment < 'tcx >]) , None , }
};
}
