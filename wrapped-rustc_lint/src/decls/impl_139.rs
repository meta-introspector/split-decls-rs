macro_rules! deps {
    () => {
        DanglingPointers!();
        DanglingPointerReturnSearcher!();
        DanglingPointerSearcher!();
        LateContext!();
        DanglingPointerLocalContext!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for DanglingPointers { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : FnKind < 'tcx > , fn_decl : & 'tcx FnDecl < 'tcx > , body : & 'tcx Body < 'tcx > , _ : Span , def_id : LocalDefId ,) { DanglingPointerSearcher { cx , inside_call_args : false } . visit_body (body) ; if let FnRetTy :: Return (ret_ty) = & fn_decl . output && let TyKind :: Ptr (_) = ret_ty . kind { let ty = match cx . tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: FnDef (..) => cx . tcx . fn_sig (def_id) . instantiate_identity () , ty :: Closure (_ , args) => args . as_closure () . sig () , _ => return , } ; let ty = ty . output () ; let ty = cx . tcx . instantiate_bound_regions_with_erased (ty) ; let inner_ty = match ty . kind () { ty :: RawPtr (inner_ty , _) => * inner_ty , _ => return , } ; if cx . tcx . layout_of (cx . typing_env () . as_query_input (inner_ty)) . is_ok_and (| layout | ! layout . is_1zst ()) { let dcx = & DanglingPointerLocalContext { body : def_id , fn_ret : ty , fn_ret_span : ret_ty . span , fn_ret_inner : inner_ty , fn_kind : match fn_kind { FnKind :: ItemFn (..) => "function" , FnKind :: Method (..) => "method" , FnKind :: Closure => "closure" , } , } ; DanglingPointerReturnSearcher { cx , dcx } . visit_body (body) ; if let ExprKind :: Block (block , None) = & body . value . kind && let innermost_block = block . innermost_block () && let Some (expr) = innermost_block . expr { lint_addr_of_local (cx , dcx , expr) ; } } } } }
    };
}

impl_139!();