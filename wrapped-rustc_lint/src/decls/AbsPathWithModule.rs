macro_rules! deps {
    () => {
        AbsPathWithModuleSugg!();
    };
}

macro_rules! AbsPathWithModule {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_abs_path_with_module)] pub (crate) struct AbsPathWithModule { # [subdiagnostic] pub sugg : AbsPathWithModuleSugg , }
    };
}

AbsPathWithModule!();