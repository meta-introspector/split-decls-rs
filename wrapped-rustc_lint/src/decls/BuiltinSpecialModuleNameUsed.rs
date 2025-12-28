macro_rules! BuiltinSpecialModuleNameUsed {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum BuiltinSpecialModuleNameUsed { # [diag (lint_builtin_special_module_name_used_lib)] # [note] # [help] Lib , # [diag (lint_builtin_special_module_name_used_main)] # [note] Main , }
    };
}

BuiltinSpecialModuleNameUsed!();