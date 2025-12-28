macro_rules! deps {
    () => {
        LossyProvenanceInt2PtrSuggestion!();
    };
}

macro_rules! LossyProvenanceInt2Ptr {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (hir_typeck_lossy_provenance_int2ptr)] # [help] pub (crate) struct LossyProvenanceInt2Ptr < 'tcx > { pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [subdiagnostic] pub sugg : LossyProvenanceInt2PtrSuggestion , }
    };
}

LossyProvenanceInt2Ptr!()