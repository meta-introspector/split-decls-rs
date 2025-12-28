macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! is_single_call_in_arm {
    () => {
        deps!();
        fn is_single_call_in_arm < 'tcx > (cx : & LateContext < 'tcx > , arg : & 'tcx Expr < '_ > , drop_expr : & 'tcx Expr < '_ > ,) -> bool { if arg . can_have_side_effects () { if let Node :: Arm (Arm { body , .. }) = cx . tcx . parent_hir_node (drop_expr . hir_id) { return body . hir_id == drop_expr . hir_id ; } } false }
    };
}

is_single_call_in_arm!()