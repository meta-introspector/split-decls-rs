macro_rules! deps {
    () => {
        Stage!();
        RustcLayoutScalarValidRangeEnd!();
        AllowedTargets!();
        AttributeOrder!();
        AcceptContext!();
        OnDuplicate!();
        SingleAttributeParser!();
        ArgParser!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for RustcLayoutScalarValidRangeEnd { const PATH : & 'static [Symbol] = & [sym :: rustc_layout_scalar_valid_range_end] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["end"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { parse_single_integer (cx , args) . map (| n | AttributeKind :: RustcLayoutScalarValidRangeEnd (Box :: new (n) , cx . attr_span)) } }
    };
}

impl_175!();