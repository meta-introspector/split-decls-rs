macro_rules! deps {
    () => {
        LateContext!();
        IntegerToPtrTransmutesSuggestion!();
        IntegerToPtrTransmutes!();
    };
}

macro_rules! check_int_to_ptr_transmute {
    () => {
        deps!();
        # [doc = " Check for transmutes from integer to pointers (*const/*mut and &/&mut)."] # [doc = ""] # [doc = " Using the resulting pointers would be undefined behavior."] fn check_int_to_ptr_transmute < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx > , arg : & 'tcx hir :: Expr < 'tcx > , src : Ty < 'tcx > , dst : Ty < 'tcx > ,) { if ! matches ! (src . kind () , ty :: Uint (_) | ty :: Int (_)) { return ; } let (ty :: Ref (_ , inner_ty , mutbl) | ty :: RawPtr (inner_ty , mutbl)) = dst . kind () else { return ; } ; if matches ! (arg . kind , hir :: ExprKind :: Lit (hir :: Lit { node : LitKind :: Int (v , _) , .. }) if v == 0) { return ; } let Ok (layout_inner_ty) = cx . tcx . layout_of (cx . typing_env () . as_query_input (* inner_ty)) else { return ; } ; if layout_inner_ty . is_1zst () { return ; } let suffix = if mutbl . is_mut () { "_mut" } else { "" } ; cx . tcx . emit_node_span_lint (INTEGER_TO_PTR_TRANSMUTES , expr . hir_id , expr . span , IntegerToPtrTransmutes { suggestion : if layout_inner_ty . is_sized () { Some (if dst . is_ref () { IntegerToPtrTransmutesSuggestion :: ToRef { dst : * inner_ty , suffix , ref_mutbl : mutbl . prefix_str () , start_call : expr . span . shrink_to_lo () . until (arg . span) , } } else { IntegerToPtrTransmutesSuggestion :: ToPtr { dst : * inner_ty , suffix , start_call : expr . span . shrink_to_lo () . until (arg . span) , } }) } else { None } , } ,) ; }
    };
}

check_int_to_ptr_transmute!();