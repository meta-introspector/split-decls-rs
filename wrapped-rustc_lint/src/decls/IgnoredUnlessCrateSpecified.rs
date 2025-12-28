macro_rules! IgnoredUnlessCrateSpecified {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_ignored_unless_crate_specified)] pub (crate) struct IgnoredUnlessCrateSpecified < 'a > { pub level : & 'a str , pub name : Symbol , }
    };
}

IgnoredUnlessCrateSpecified!()