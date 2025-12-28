macro_rules! deps {
    () => {
        ShadowedIntoIterDiagSub!();
    };
}

macro_rules! ShadowedIntoIterDiag {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_shadowed_into_iter)] pub (crate) struct ShadowedIntoIterDiag { pub target : & 'static str , pub edition : & 'static str , # [suggestion (lint_use_iter_suggestion , code = "iter" , applicability = "machine-applicable")] pub suggestion : Span , # [subdiagnostic] pub sub : Option < ShadowedIntoIterDiagSub > , }
    };
}

ShadowedIntoIterDiag!()