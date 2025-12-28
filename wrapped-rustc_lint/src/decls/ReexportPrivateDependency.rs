macro_rules! ReexportPrivateDependency {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_reexport_private_dependency)] pub (crate) struct ReexportPrivateDependency { pub name : String , pub kind : String , pub krate : Symbol , }
    };
}

ReexportPrivateDependency!();