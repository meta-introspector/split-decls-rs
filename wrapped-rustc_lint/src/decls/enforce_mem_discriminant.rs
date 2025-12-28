macro_rules! deps {
    () => {
        LateContext!();
        EnumIntrinsicsMemDiscriminate!();
    };
}

macro_rules! enforce_mem_discriminant {
    () => {
        deps!();
        fn enforce_mem_discriminant (cx : & LateContext < '_ > , func_expr : & hir :: Expr < '_ > , expr_span : Span , args_span : Span ,) { let ty_param = cx . typeck_results () . node_args (func_expr . hir_id) . type_at (0) ; if is_non_enum (ty_param) { cx . emit_span_lint (ENUM_INTRINSICS_NON_ENUMS , expr_span , EnumIntrinsicsMemDiscriminate { ty_param , note : args_span } ,) ; } }
    };
}

enforce_mem_discriminant!();