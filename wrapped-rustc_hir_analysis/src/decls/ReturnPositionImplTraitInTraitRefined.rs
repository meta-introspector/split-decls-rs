macro_rules! ReturnPositionImplTraitInTraitRefined {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_analysis_rpitit_refined)] # [note] # [note (hir_analysis_feedback_note)] pub (crate) struct ReturnPositionImplTraitInTraitRefined < 'tcx > { # [suggestion (applicability = "maybe-incorrect" , code = "{pre}{return_ty}{post}")] pub impl_return_span : Span , # [label] pub trait_return_span : Option < Span > , # [label (hir_analysis_unmatched_bound_label)] pub unmatched_bound : Option < Span > , pub pre : & 'static str , pub post : & 'static str , pub return_ty : Ty < 'tcx > , }
    };
}

ReturnPositionImplTraitInTraitRefined!();