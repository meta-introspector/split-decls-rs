macro_rules! BuiltinTrivialBounds {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_trivial_bounds)] pub (crate) struct BuiltinTrivialBounds < 'a > { pub predicate_kind_name : & 'a str , pub predicate : Clause < 'a > , }
    };
}

BuiltinTrivialBounds!();