macro_rules! MacroExpandedMacroExportsAccessedByAbsolutePaths {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_macro_expanded_macro_exports_accessed_by_absolute_paths)] pub (crate) struct MacroExpandedMacroExportsAccessedByAbsolutePaths { # [note] pub definition : Span , }
    };
}

MacroExpandedMacroExportsAccessedByAbsolutePaths!();