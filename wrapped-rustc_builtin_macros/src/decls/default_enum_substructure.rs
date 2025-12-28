macro_rules! deps {
    () => {
        BlockOrExpr!();
    };
}

macro_rules! default_enum_substructure {
    () => {
        deps!();
        fn default_enum_substructure (cx : & ExtCtxt < '_ > , trait_span : Span , enum_def : & EnumDef , item_span : Span ,) -> BlockOrExpr { let expr = match try { let default_variant = extract_default_variant (cx , enum_def , trait_span , item_span) ? ; validate_default_attribute (cx , default_variant) ? ; default_variant } { Ok (default_variant) => { match & default_variant . data { VariantData :: Unit (_) => cx . expr_path (cx . path (default_variant . span , vec ! [Ident :: new (kw :: SelfUpper , default_variant . span) , default_variant . ident] ,)) , VariantData :: Struct { fields , .. } => { let default_fields = fields . iter () . map (| field | { cx . field_imm (field . span , field . ident . unwrap () , match & field . default { None => default_call (cx , field . span) , Some (val) => cx . expr (val . value . span , ast :: ExprKind :: ConstBlock (val . clone ()) ,) , } ,) }) . collect () ; let path = cx . path (default_variant . span , vec ! [Ident :: new (kw :: SelfUpper , default_variant . span) , default_variant . ident ,] ,) ; cx . expr_struct (default_variant . span , path , default_fields) } VariantData :: Tuple (..) => { cx . dcx () . bug ("encountered tuple variant annotated with `#[default]`") } } } Err (guar) => DummyResult :: raw_expr (trait_span , Some (guar)) , } ; BlockOrExpr :: new_expr (expr) }
    };
}

default_enum_substructure!();