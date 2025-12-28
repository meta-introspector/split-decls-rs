macro_rules! deps {
    () => {
        UnknownCrateTypesSub!();
    };
}

macro_rules! UnknownCrateTypes {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_invalid_crate_type_value)] pub (crate) struct UnknownCrateTypes { # [subdiagnostic] pub sugg : Option < UnknownCrateTypesSub > , }
    };
}

UnknownCrateTypes!();