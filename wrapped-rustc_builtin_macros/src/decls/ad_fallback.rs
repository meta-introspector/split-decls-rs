macro_rules! ad_fallback {
    () => {
        mod ad_fallback { use super :: * ; # [derive (Diagnostic)] # [diag (builtin_macros_autodiff_not_build)] pub (crate) struct AutoDiffSupportNotBuild { # [primary_span] pub (crate) span : Span , } }
    };
}

ad_fallback!();