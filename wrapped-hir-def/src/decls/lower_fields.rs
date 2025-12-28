macro_rules! deps {
    () => {
        ExpressionStoreSourceMap!();
        Attrs!();
        ModuleId!();
        FieldData!();
        ExpressionStore!();
        Visibility!();
        ExpressionStoreDiagnostics!();
        DefDatabase!();
        RawVisibility!();
        Item!();
    };
}

macro_rules! lower_fields {
    () => {
        deps!();
        fn lower_fields < Field : ast :: HasAttrs + ast :: HasVisibility > (db : & dyn DefDatabase , module : ModuleId , fields : InFile < impl Iterator < Item = (Option < ast :: Type > , Field) > > , mut field_name : impl FnMut (usize , & Field) -> Name , override_visibility : Option < Option < ast :: Visibility > > ,) -> Option < (Arena < FieldData > , ExpressionStore , ExpressionStoreSourceMap) > { let cfg_options = module . krate . cfg_options (db) ; let mut col = ExprCollector :: new (db , module , fields . file_id) ; let override_visibility = override_visibility . map (| vis | { LazyCell :: new (| | { let span_map = db . span_map (fields . file_id) ; visibility_from_ast (db , vis , & mut | range | span_map . span_for_range (range) . ctx) }) }) ; let mut arena = Arena :: new () ; let mut idx = 0 ; let mut has_fields = false ; for (ty , field) in fields . value { has_fields = true ; match Attrs :: is_cfg_enabled_for (db , & field , col . span_map () , cfg_options) { Ok (()) => { let type_ref = col . lower_type_ref_opt (ty , & mut ExprCollector :: impl_trait_error_allocator) ; let visibility = override_visibility . as_ref () . map_or_else (| | { visibility_from_ast (db , field . visibility () , & mut | range | { col . span_map () . span_for_range (range) . ctx }) } , | it | RawVisibility :: clone (it) ,) ; let is_unsafe = field . syntax () . children_with_tokens () . filter_map (NodeOrToken :: into_token) . any (| token | token . kind () == T ! [unsafe]) ; let name = field_name (idx , & field) ; arena . alloc (FieldData { name , type_ref , visibility , is_unsafe }) ; idx += 1 ; } Err (cfg) => { col . store . diagnostics . push (crate :: expr_store :: ExpressionStoreDiagnostics :: InactiveCode { node : InFile :: new (fields . file_id , SyntaxNodePtr :: new (field . syntax ())) , cfg , opts : cfg_options . clone () , } ,) ; } } } if ! has_fields { return None ; } let (store , source_map) = col . store . finish () ; arena . shrink_to_fit () ; Some ((arena , store , source_map)) }
    };
}

lower_fields!()