macro_rules! GenericsArgsErrExtend {
    () => {
        # [doc = " Used for generics args error extend."] pub enum GenericsArgsErrExtend < 'tcx > { EnumVariant { qself : & 'tcx hir :: Ty < 'tcx > , assoc_segment : & 'tcx hir :: PathSegment < 'tcx > , adt_def : AdtDef < 'tcx > , } , OpaqueTy , PrimTy (hir :: PrimTy) , SelfTyAlias { def_id : DefId , span : Span , } , SelfTyParam (Span) , Param (DefId) , DefVariant (& 'tcx [hir :: PathSegment < 'tcx >]) , None , }
    };
}

GenericsArgsErrExtend!();