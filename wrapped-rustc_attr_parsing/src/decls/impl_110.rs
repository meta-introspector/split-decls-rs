macro_rules! deps {
    () => {
        OnDuplicate!();
        Stage!();
        NoArgsAttributeParser!();
        PassByValueParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for PassByValueParser { const PATH : & [Symbol] = & [sym :: rustc_pass_by_value] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Enum) , Allow (Target :: TyAlias) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: PassByValue ; }
    };
}

impl_110!()