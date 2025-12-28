macro_rules! EnumIntrinsicsMemVariant {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_enum_intrinsics_mem_variant)] # [note] pub (crate) struct EnumIntrinsicsMemVariant < 'a > { pub ty_param : Ty < 'a > , }
    };
}

EnumIntrinsicsMemVariant!()