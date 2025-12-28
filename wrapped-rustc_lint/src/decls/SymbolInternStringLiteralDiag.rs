macro_rules! SymbolInternStringLiteralDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_symbol_intern_string_literal)] # [help] pub (crate) struct SymbolInternStringLiteralDiag ;
    };
}

SymbolInternStringLiteralDiag!();