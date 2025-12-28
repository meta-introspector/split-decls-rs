macro_rules! deps {
    () => {
        OnDuplicate!();
        AllowedTargets!();
        NoArgsAttributeParser!();
        ParenSugarParser!();
        Stage!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ParenSugarParser { const PATH : & [Symbol] = & [sym :: rustc_paren_sugar] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ParenSugar ; }
    };
}

impl_206!();