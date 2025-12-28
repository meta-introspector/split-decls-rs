macro_rules! TypeIrDirectUse {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_type_ir_direct_use)] # [note] pub (crate) struct TypeIrDirectUse ;
    };
}

TypeIrDirectUse!();