macro_rules! BuiltinMutablesTransmutes {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_builtin_mutable_transmutes)] pub (crate) struct BuiltinMutablesTransmutes ;
    };
}

BuiltinMutablesTransmutes!()