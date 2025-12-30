// Generated macro for BuiltinUnsafe (enum)
macro_rules! Depcrate_lintsBuiltinUnsafe {
() => {
// Module: crate::lints
// Provides: {"BuiltinUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum BuiltinUnsafe { # [diag (lint_builtin_allow_internal_unsafe)] AllowInternalUnsafe , # [diag (lint_builtin_unsafe_block)] UnsafeBlock , # [diag (lint_builtin_unsafe_extern_block)] UnsafeExternBlock , # [diag (lint_builtin_unsafe_trait)] UnsafeTrait , # [diag (lint_builtin_unsafe_impl)] UnsafeImpl , # [diag (lint_builtin_no_mangle_fn)] # [note (lint_builtin_overridden_symbol_name)] NoMangleFn , # [diag (lint_builtin_export_name_fn)] # [note (lint_builtin_overridden_symbol_name)] ExportNameFn , # [diag (lint_builtin_link_section_fn)] # [note (lint_builtin_overridden_symbol_section)] LinkSectionFn , # [diag (lint_builtin_no_mangle_static)] # [note (lint_builtin_overridden_symbol_name)] NoMangleStatic , # [diag (lint_builtin_export_name_static)] # [note (lint_builtin_overridden_symbol_name)] ExportNameStatic , # [diag (lint_builtin_link_section_static)] # [note (lint_builtin_overridden_symbol_section)] LinkSectionStatic , # [diag (lint_builtin_no_mangle_method)] # [note (lint_builtin_overridden_symbol_name)] NoMangleMethod , # [diag (lint_builtin_export_name_method)] # [note (lint_builtin_overridden_symbol_name)] ExportNameMethod , # [diag (lint_builtin_decl_unsafe_fn)] DeclUnsafeFn , # [diag (lint_builtin_decl_unsafe_method)] DeclUnsafeMethod , # [diag (lint_builtin_impl_unsafe_method)] ImplUnsafeMethod , # [diag (lint_builtin_global_asm)] # [note (lint_builtin_global_macro_unsafety)] GlobalAsm , }
};
}
