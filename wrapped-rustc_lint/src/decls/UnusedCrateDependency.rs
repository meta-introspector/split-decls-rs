macro_rules! UnusedCrateDependency {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_crate_dependency)] # [help] pub (crate) struct UnusedCrateDependency { pub extern_crate : Symbol , pub local_crate : Symbol , }
    };
}

UnusedCrateDependency!()