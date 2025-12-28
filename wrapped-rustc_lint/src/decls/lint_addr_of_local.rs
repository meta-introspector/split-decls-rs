macro_rules! deps {
    () => {
        LateContext!();
        DanglingPointerLocalContext!();
        DanglingPointersFromLocals!();
    };
}

macro_rules! lint_addr_of_local {
    () => {
        deps!();
        # [doc = " Look for `&<path_to_local_in_same_body>` pattern and emit lint for it"] fn lint_addr_of_local < 'a > (cx : & LateContext < 'a > , dcx : & DanglingPointerLocalContext < 'a > , expr : & 'a Expr < 'a > ,) { let (inner , _) = super :: utils :: peel_casts (cx , expr) ; if let ExprKind :: AddrOf (_ , _ , inner_of) = inner . kind && let ExprKind :: Path (ref qpath) = inner_of . peel_blocks () . kind && let Res :: Local (from) = cx . qpath_res (qpath , inner_of . hir_id) && cx . tcx . hir_enclosing_body_owner (from) == dcx . body { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_LOCALS , expr . hir_id , expr . span , DanglingPointersFromLocals { ret_ty : dcx . fn_ret , ret_ty_span : dcx . fn_ret_span , fn_kind : dcx . fn_kind , local_var : cx . tcx . hir_span (from) , local_var_name : cx . tcx . hir_ident (from) , local_var_ty : dcx . fn_ret_inner , created_at : (expr . hir_id != inner . hir_id) . then_some (inner . span) , } ,) ; } }
    };
}

lint_addr_of_local!()