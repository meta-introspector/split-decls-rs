macro_rules! deps {
    () => {
        SingleAttributeParser!();
        Stage!();
        OnDuplicate!();
        AllowedTargets!();
        ArgParser!();
        DummyParser!();
        AttributeOrder!();
        AcceptContext!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < S : Stage > SingleAttributeParser < S > for DummyParser { const PATH : & [Symbol] = & [sym :: rustc_dummy] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (Word) ; fn convert (_ : & mut AcceptContext < '_ , '_ , S > , _ : & ArgParser < '_ >) -> Option < AttributeKind > { Some (AttributeKind :: Dummy) } }
    };
}

impl_78!()