macro_rules! deps {
    () => {
        AddBound!();
    };
}

macro_rules! OpaqueHiddenInferredBoundLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_opaque_hidden_inferred_bound)] struct OpaqueHiddenInferredBoundLint < 'tcx > { ty : Ty < 'tcx > , proj_ty : Ty < 'tcx > , # [label (lint_specifically)] assoc_pred_span : Span , # [subdiagnostic] add_bound : Option < AddBound < 'tcx > > , }
    };
}

OpaqueHiddenInferredBoundLint!();