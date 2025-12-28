macro_rules! deps {
    () => {
        EnumIntrinsicsMemVariant!();
        LateContext!();
    };
}

macro_rules! enforce_mem_variant_count {
    () => {
        deps!();
        fn enforce_mem_variant_count (cx : & LateContext < '_ > , func_expr : & hir :: Expr < '_ > , span : Span) { let ty_param = cx . typeck_results () . node_args (func_expr . hir_id) . type_at (0) ; if is_non_enum (ty_param) { cx . emit_span_lint (ENUM_INTRINSICS_NON_ENUMS , span , EnumIntrinsicsMemVariant { ty_param }) ; } }
    };
}

enforce_mem_variant_count!()