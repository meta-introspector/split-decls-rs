// Generated macro for ResolvedPatKind (enum)
macro_rules! Depcrate_patResolvedPatKind {
() => {
// Module: crate::pat
// Provides: {"ResolvedPatKind"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] enum ResolvedPatKind < 'tcx > { Path { res : Res , pat_res : Res , segments : & 'tcx [hir :: PathSegment < 'tcx >] } , Struct { variant : & 'tcx VariantDef } , TupleStruct { res : Res , variant : & 'tcx VariantDef } , }
};
}
