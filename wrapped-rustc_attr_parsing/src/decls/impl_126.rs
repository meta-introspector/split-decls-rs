macro_rules! deps {
    () => {
        AllowedTargets!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        Stage!();
        AllowInternalUnsafeParser!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for AllowInternalUnsafeParser { const PATH : & [Symbol] = & [sym :: allow_internal_unsafe] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: MacroDef) , Warn (Target :: Field) , Warn (Target :: Arm) ,]) ; const CREATE : fn (Span) -> AttributeKind = | span | AttributeKind :: AllowInternalUnsafe (span) ; }
    };
}

impl_126!();