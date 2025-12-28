macro_rules! DanglingPointersFromLocals {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_dangling_pointers_from_locals)] # [note] pub (crate) struct DanglingPointersFromLocals < 'tcx > { pub ret_ty : Ty < 'tcx > , # [label (lint_ret_ty)] pub ret_ty_span : Span , pub fn_kind : & 'static str , # [label (lint_local_var)] pub local_var : Span , pub local_var_name : Ident , pub local_var_ty : Ty < 'tcx > , # [label (lint_created_at)] pub created_at : Option < Span > , }
    };
}

DanglingPointersFromLocals!()