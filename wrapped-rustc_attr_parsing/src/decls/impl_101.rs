macro_rules! deps {
    () => {
        AttributeOrder!();
        AllowedTargets!();
        SingleAttributeParser!();
        LinkOrdinalOutOfRange!();
        LinkOrdinalParser!();
        Stage!();
        OnDuplicate!();
        AcceptContext!();
        ArgParser!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for LinkOrdinalParser { const PATH : & [Symbol] = & [sym :: link_ordinal] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepOutermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: ForeignFn) , Allow (Target :: ForeignStatic) , Warn (Target :: MacroCall) ,]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["ordinal"] , "https://doc.rust-lang.org/reference/items/external-blocks.html#the-link_ordinal-attribute") ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let ordinal = parse_single_integer (cx , args) ? ; let Ok (ordinal) = ordinal . try_into () else { cx . emit_err (LinkOrdinalOutOfRange { span : cx . attr_span , ordinal }) ; return None ; } ; Some (LinkOrdinal { ordinal , span : cx . attr_span }) } }
    };
}

impl_101!()