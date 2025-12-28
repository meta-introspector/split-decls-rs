macro_rules! deps {
    () => {
        OverruledAttributeSub!();
    };
}

macro_rules! OverruledAttribute {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (lint_overruled_attribute , code = E0453)] pub (crate) struct OverruledAttribute < 'a > { # [primary_span] pub span : Span , # [label] pub overruled : Span , pub lint_level : & 'a str , pub lint_source : Symbol , # [subdiagnostic] pub sub : OverruledAttributeSub , }
    };
}

OverruledAttribute!()