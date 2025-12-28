macro_rules! deps {
    () => {
        Stage!();
        AcceptContext!();
        CustomMirParser!();
        SingleAttributeParser!();
        AllowedTargets!();
        AttributeOrder!();
        OnDuplicate!();
        ArgParser!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for CustomMirParser { const PATH : & [rustc_span :: Symbol] = & [sym :: custom_mir] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & [r#"dialect = "...", phase = "...""#]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let mut dialect = None ; let mut phase = None ; let mut failed = false ; for item in list . mixed () { let Some (meta_item) = item . meta_item () else { cx . expected_name_value (item . span () , None) ; failed = true ; break ; } ; if let Some (arg) = meta_item . word_is (sym :: dialect) { extract_value (cx , sym :: dialect , arg , meta_item . span () , & mut dialect , & mut failed) ; } else if let Some (arg) = meta_item . word_is (sym :: phase) { extract_value (cx , sym :: phase , arg , meta_item . span () , & mut phase , & mut failed) ; } else if let Some (word) = meta_item . path () . word () { let word = word . to_string () ; cx . unknown_key (meta_item . span () , word , & ["dialect" , "phase"]) ; failed = true ; } else { cx . expected_name_value (meta_item . span () , None) ; failed = true ; } ; } let dialect = parse_dialect (cx , dialect , & mut failed) ; let phase = parse_phase (cx , phase , & mut failed) ; if failed { return None ; } Some (AttributeKind :: CustomMir (dialect , phase , cx . attr_span)) } }
    };
}

impl_152!();