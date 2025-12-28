macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        Stage!();
        CoinductiveParser!();
        OnDuplicate!();
        AllowedTargets!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for CoinductiveParser { const PATH : & [Symbol] = & [sym :: rustc_coinductive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Coinductive ; }
    };
}

impl_222!();