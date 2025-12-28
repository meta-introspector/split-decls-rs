macro_rules! deps {
    () => {
        BlockOrExpr!();
        FieldInfo!();
        Substructure!();
    };
}

macro_rules! cs_clone {
    () => {
        deps!();
        fn cs_clone (name : & str , cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ > ,) -> BlockOrExpr { let ctor_path ; let all_fields ; let fn_path = cx . std_path (& [sym :: clone , sym :: Clone , sym :: clone]) ; let subcall = | cx : & ExtCtxt < '_ > , field : & FieldInfo | { let args = thin_vec ! [field . self_expr . clone ()] ; cx . expr_call_global (field . span , fn_path . clone () , args) } ; let vdata ; match substr . fields { Struct (vdata_ , af) => { ctor_path = cx . path (trait_span , vec ! [substr . type_ident]) ; all_fields = af ; vdata = * vdata_ ; } EnumMatching (.. , variant , af) => { ctor_path = cx . path (trait_span , vec ! [substr . type_ident , variant . ident]) ; all_fields = af ; vdata = & variant . data ; } EnumDiscr (..) | AllFieldlessEnum (..) => { cx . dcx () . span_bug (trait_span , format ! ("enum discriminants in `derive({name})`" ,)) } StaticEnum (..) | StaticStruct (..) => { cx . dcx () . span_bug (trait_span , format ! ("associated function in `derive({name})`")) } } let expr = match * vdata { VariantData :: Struct { .. } => { let fields = all_fields . iter () . map (| field | { let Some (ident) = field . name else { cx . dcx () . span_bug (trait_span , format ! ("unnamed field in normal struct in `derive({name})`" ,) ,) ; } ; let call = subcall (cx , field) ; cx . field_imm (field . span , ident , call) }) . collect :: < ThinVec < _ > > () ; cx . expr_struct (trait_span , ctor_path , fields) } VariantData :: Tuple (..) => { let subcalls = all_fields . iter () . map (| f | subcall (cx , f)) . collect () ; let path = cx . expr_path (ctor_path) ; cx . expr_call (trait_span , path , subcalls) } VariantData :: Unit (..) => cx . expr_path (ctor_path) , } ; BlockOrExpr :: new_expr (expr) }
    };
}

cs_clone!();