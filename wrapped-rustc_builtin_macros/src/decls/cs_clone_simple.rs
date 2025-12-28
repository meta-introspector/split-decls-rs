macro_rules! deps {
    () => {
        BlockOrExpr!();
        Substructure!();
    };
}

macro_rules! cs_clone_simple {
    () => {
        deps!();
        fn cs_clone_simple (name : & str , cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ > , is_union : bool ,) -> BlockOrExpr { let mut stmts = ThinVec :: new () ; let mut seen_type_names = FxHashSet :: default () ; let mut process_variant = | variant : & VariantData | { for field in variant . fields () { if let Some (name) = field . ty . kind . is_simple_path () && ! seen_type_names . insert (name) { } else { super :: assert_ty_bounds (cx , & mut stmts , field . ty . clone () , field . span , & [sym :: clone , sym :: AssertParamIsClone] ,) ; } } } ; if is_union { let self_ty = cx . ty_path (cx . path_ident (trait_span , Ident :: with_dummy_span (kw :: SelfUpper))) ; super :: assert_ty_bounds (cx , & mut stmts , self_ty , trait_span , & [sym :: clone , sym :: AssertParamIsCopy] ,) ; } else { match * substr . fields { StaticStruct (vdata , ..) => { process_variant (vdata) ; } StaticEnum (enum_def , ..) => { for variant in & enum_def . variants { process_variant (& variant . data) ; } } _ => cx . dcx () . span_bug (trait_span , format ! ("unexpected substructure in simple `derive({name})`") ,) , } } BlockOrExpr :: new_mixed (stmts , Some (cx . expr_deref (trait_span , cx . expr_self (trait_span)))) }
    };
}

cs_clone_simple!();