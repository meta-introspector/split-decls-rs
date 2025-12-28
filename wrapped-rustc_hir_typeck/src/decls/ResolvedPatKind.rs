macro_rules! ResolvedPatKind {
    () => {
        # [derive (Clone , Copy , Debug)] enum ResolvedPatKind < 'tcx > { Path { res : Res , pat_res : Res , segments : & 'tcx [hir :: PathSegment < 'tcx >] } , Struct { variant : & 'tcx VariantDef } , TupleStruct { res : Res , variant : & 'tcx VariantDef } , }
    };
}

ResolvedPatKind!();