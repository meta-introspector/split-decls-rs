macro_rules! deps {
    () => {
        OnDuplicate!();
        AttributeOrder!();
        ArgParser!();
        AcceptContext!();
        Stage!();
        SingleAttributeParser!();
        RustcLayoutScalarValidRangeStart!();
        AllowedTargets!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for RustcLayoutScalarValidRangeStart { const PATH : & 'static [Symbol] = & [sym :: rustc_layout_scalar_valid_range_start] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["start"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { parse_single_integer (cx , args) . map (| n | AttributeKind :: RustcLayoutScalarValidRangeStart (Box :: new (n) , cx . attr_span)) } }
    };
}

impl_173!()