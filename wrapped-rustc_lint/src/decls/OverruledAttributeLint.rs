macro_rules! deps {
    () => {
        OverruledAttributeSub!();
    };
}

macro_rules! OverruledAttributeLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_overruled_attribute)] pub (crate) struct OverruledAttributeLint < 'a > { # [label] pub overruled : Span , pub lint_level : & 'a str , pub lint_source : Symbol , # [subdiagnostic] pub sub : OverruledAttributeSub , }
    };
}

OverruledAttributeLint!()