macro_rules! deps {
    () => {
        AllowedTargets!();
        SingleAttributeParser!();
        ArgParser!();
        Stage!();
        AcceptContext!();
        PatternComplexityLimitParser!();
        AttributeOrder!();
        OnDuplicate!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for PatternComplexityLimitParser { const PATH : & [Symbol] = & [sym :: pattern_complexity_limit] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const TEMPLATE : AttributeTemplate = template ! (NameValueStr : "N") ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let ArgParser :: NameValue (nv) = args else { cx . expected_name_value (cx . attr_span , None) ; return None ; } ; Some (AttributeKind :: PatternComplexityLimit { limit : cx . parse_limit_int (nv) ? , attr_span : cx . attr_span , limit_span : nv . value_span , }) } }
    };
}

impl_67!();