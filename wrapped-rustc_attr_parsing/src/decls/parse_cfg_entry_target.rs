macro_rules! deps {
    () => {
        Stage!();
        AcceptContext!();
        MetaItemListParser!();
        CfgPredicateIdentifier!();
    };
}

macro_rules! parse_cfg_entry_target {
    () => {
        deps!();
        fn parse_cfg_entry_target < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , list : & MetaItemListParser < '_ > , meta_span : Span ,) -> Option < CfgEntry > { if let Some (features) = cx . features_option () && ! features . cfg_target_compact () { feature_err (cx . sess () , sym :: cfg_target_compact , meta_span , fluent_generated :: attr_parsing_unstable_cfg_target_compact ,) . emit () ; } let mut result = ThinVec :: new () ; for sub_item in list . mixed () { let Some (sub_item) = sub_item . meta_item () else { cx . expected_name_value (sub_item . span () , None) ; continue ; } ; let Some (nv) = sub_item . args () . name_value () else { cx . expected_name_value (sub_item . span () , None) ; continue ; } ; let Some (name) = sub_item . path () . word_sym () else { cx . emit_err (session_diagnostics :: CfgPredicateIdentifier { span : sub_item . path () . span () , }) ; return None ; } ; let name = Symbol :: intern (& format ! ("target_{name}")) ; if let Some (cfg) = parse_name_value (name , sub_item . path () . span () , Some (nv) , sub_item . span () , cx) { result . push (cfg) ; } } Some (CfgEntry :: All (result , list . span)) }
    };
}

parse_cfg_entry_target!()