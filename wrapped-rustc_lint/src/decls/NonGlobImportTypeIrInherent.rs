macro_rules! NonGlobImportTypeIrInherent {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_non_glob_import_type_ir_inherent)] pub (crate) struct NonGlobImportTypeIrInherent { # [suggestion (code = "{snippet}" , applicability = "maybe-incorrect")] pub suggestion : Option < Span > , pub snippet : & 'static str , }
    };
}

NonGlobImportTypeIrInherent!();