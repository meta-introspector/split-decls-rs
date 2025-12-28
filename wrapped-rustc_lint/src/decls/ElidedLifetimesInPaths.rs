macro_rules! ElidedLifetimesInPaths {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_hidden_lifetime_parameters)] pub (crate) struct ElidedLifetimesInPaths { # [subdiagnostic] pub subdiag : ElidedLifetimeInPathSubdiag , }
    };
}

ElidedLifetimesInPaths!()