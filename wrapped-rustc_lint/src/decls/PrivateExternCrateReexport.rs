macro_rules! PrivateExternCrateReexport {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_private_extern_crate_reexport , code = E0365)] pub (crate) struct PrivateExternCrateReexport { pub ident : Ident , # [suggestion (code = "pub " , style = "verbose" , applicability = "maybe-incorrect")] pub sugg : Span , }
    };
}

PrivateExternCrateReexport!()