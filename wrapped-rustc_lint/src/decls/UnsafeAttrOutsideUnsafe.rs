macro_rules! deps {
    () => {
        UnsafeAttrOutsideUnsafeSuggestion!();
    };
}

macro_rules! UnsafeAttrOutsideUnsafe {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unsafe_attr_outside_unsafe)] pub (crate) struct UnsafeAttrOutsideUnsafe { # [label] pub span : Span , # [subdiagnostic] pub suggestion : UnsafeAttrOutsideUnsafeSuggestion , }
    };
}

UnsafeAttrOutsideUnsafe!()