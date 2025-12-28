macro_rules! ProcMacroDeriveResolutionFallback {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_proc_macro_derive_resolution_fallback)] pub (crate) struct ProcMacroDeriveResolutionFallback { # [label] pub span : Span , pub ns_descr : & 'static str , pub ident : Ident , }
    };
}

ProcMacroDeriveResolutionFallback!();