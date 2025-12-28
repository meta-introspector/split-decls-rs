macro_rules! IncompatibleFeatures {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_incompatible_features)] # [help] pub (crate) struct IncompatibleFeatures { # [primary_span] pub spans : Vec < Span > , pub f1 : Symbol , pub f2 : Symbol , }
    };
}

IncompatibleFeatures!();