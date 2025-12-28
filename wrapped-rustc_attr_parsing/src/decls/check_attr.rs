macro_rules! deps {
    () => {
        Late!();
        AttributeParser!();
    };
}

macro_rules! check_attr {
    () => {
        deps!();
        pub fn check_attr (psess : & ParseSess , attr : & Attribute , id : NodeId) { if attr . is_doc_comment () || attr . has_name (sym :: cfg_trace) || attr . has_name (sym :: cfg_attr_trace) { return ; } let builtin_attr_info = attr . ident () . and_then (| ident | BUILTIN_ATTRIBUTE_MAP . get (& ident . name)) ; let builtin_attr_safety = builtin_attr_info . map (| x | x . safety) ; check_attribute_safety (psess , builtin_attr_safety , attr , id) ; match builtin_attr_info { Some (BuiltinAttribute { name , template , .. }) => { if AttributeParser :: < Late > :: is_parsed_attribute (slice :: from_ref (& name)) { return ; } match parse_meta (psess , attr) { Ok (meta) => { check_builtin_meta_item (psess , & meta , attr . style , * name , * template , false) } Err (err) => { err . emit () ; } } } _ => { let attr_item = attr . get_normal_item () ; if let AttrArgs :: Eq { .. } = attr_item . args { match parse_meta (psess , attr) { Ok (_) => { } Err (err) => { err . emit () ; } } } } } }
    };
}

check_attr!();