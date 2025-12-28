macro_rules! deps {
    () => {
        UnusedImportsSugg!();
    };
}

macro_rules! UnusedImports {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_unused_imports)] pub (crate) struct UnusedImports { # [subdiagnostic] pub sugg : UnusedImportsSugg , # [help] pub test_module_span : Option < Span > , pub span_snippets : DiagArgValue , pub num_snippets : usize , }
    };
}

UnusedImports!();