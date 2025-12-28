macro_rules! EnumIntrinsicsMemDiscriminate {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_enum_intrinsics_mem_discriminant)] pub (crate) struct EnumIntrinsicsMemDiscriminate < 'a > { pub ty_param : Ty < 'a > , # [note] pub note : Span , }
    };
}

EnumIntrinsicsMemDiscriminate!()