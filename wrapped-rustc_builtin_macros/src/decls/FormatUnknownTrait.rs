macro_rules! deps {
    () => {
        FormatUnknownTraitSugg!();
    };
}

macro_rules! FormatUnknownTrait {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_format_unknown_trait)] # [note] pub (crate) struct FormatUnknownTrait < 'a > { # [primary_span] pub (crate) span : Span , pub (crate) ty : & 'a str , # [subdiagnostic] pub (crate) suggs : Vec < FormatUnknownTraitSugg > , }
    };
}

FormatUnknownTrait!()