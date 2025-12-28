macro_rules! deps {
    () => {
        AcceptContext!();
        AttributeOrder!();
        ArgParser!();
        OnDuplicate!();
        Stage!();
        AllowedTargets!();
        SingleAttributeParser!();
        RustcObjectLifetimeDefaultParser!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for RustcObjectLifetimeDefaultParser { const PATH : & [rustc_span :: Symbol] = & [sym :: rustc_object_lifetime_default] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct)]) ; const TEMPLATE : AttributeTemplate = template ! (Word) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { if let Err (span) = args . no_args () { cx . expected_no_args (span) ; return None ; } Some (AttributeKind :: RustcObjectLifetimeDefault) } }
    };
}

impl_177!()