macro_rules! deps {
    () => {
        Stage!();
        FundamentalParser!();
        OnDuplicate!();
        AllowedTargets!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for FundamentalParser { const PATH : & [Symbol] = & [sym :: fundamental] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: Fundamental ; }
    };
}

impl_228!()