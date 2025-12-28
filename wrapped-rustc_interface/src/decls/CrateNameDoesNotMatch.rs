macro_rules! CrateNameDoesNotMatch {
    () => {
        # [derive (Diagnostic)] # [diag (interface_crate_name_does_not_match)] pub (crate) struct CrateNameDoesNotMatch { # [primary_span] pub (crate) span : Span , pub (crate) crate_name : Symbol , pub (crate) attr_crate_name : Symbol , }
    };
}

CrateNameDoesNotMatch!();