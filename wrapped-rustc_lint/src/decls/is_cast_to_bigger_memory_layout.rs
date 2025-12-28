macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! is_cast_to_bigger_memory_layout {
    () => {
        deps!();
        fn is_cast_to_bigger_memory_layout < 'tcx > (cx : & LateContext < 'tcx > , orig_expr : & 'tcx Expr < 'tcx > , mut peel_casts : impl FnMut () -> (& 'tcx Expr < 'tcx > , bool) ,) -> Option < (TyAndLayout < 'tcx > , TyAndLayout < 'tcx > , Expr < 'tcx >) > { let end_ty = cx . typeck_results () . node_type (orig_expr . hir_id) ; let ty :: RawPtr (inner_end_ty , _) = end_ty . kind () else { return None ; } ; let (e , _) = peel_casts () ; let start_ty = cx . typeck_results () . node_type (e . hir_id) ; let ty :: Ref (_ , inner_start_ty , _) = start_ty . kind () else { return None ; } ; let e_alloc = cx . expr_or_init (e) ; let e_alloc = if let ExprKind :: AddrOf (_ , _ , inner_expr) = e_alloc . kind { inner_expr } else { e_alloc } ; if let ExprKind :: Index (..) | ExprKind :: Field (..) | ExprKind :: Unary (UnOp :: Deref , ..) = e_alloc . kind { return None ; } let alloc_ty = cx . typeck_results () . node_type (e_alloc . hir_id) ; if alloc_ty . is_any_ptr () { return None ; } let from_layout = cx . layout_of (* inner_start_ty) . ok () ? ; if from_layout . is_unsized () { return None ; } let alloc_layout = cx . layout_of (alloc_ty) . ok () ? ; let to_layout = cx . layout_of (* inner_end_ty) . ok () ? ; if to_layout . layout . size () > from_layout . layout . size () && to_layout . layout . size () > alloc_layout . layout . size () { Some ((from_layout , to_layout , * e_alloc)) } else { None } }
    };
}

is_cast_to_bigger_memory_layout!()