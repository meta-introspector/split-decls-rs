macro_rules! deps {
    () => {
        LossyProvenancePtr2IntSuggestion!();
    };
}

macro_rules! LossyProvenancePtr2Int {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (hir_typeck_lossy_provenance_ptr2int)] # [help] pub (crate) struct LossyProvenancePtr2Int < 'tcx > { pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [subdiagnostic] pub sugg : LossyProvenancePtr2IntSuggestion < 'tcx > , }
    };
}

LossyProvenancePtr2Int!();