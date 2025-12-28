macro_rules! TyParamSomeLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (hir_analysis_ty_param_some , code = E0210)] # [note] pub (crate) struct TyParamSomeLint { # [label] pub span : Span , # [note (hir_analysis_only_note)] pub note : () , pub param : Ident , }
    };
}

TyParamSomeLint!()