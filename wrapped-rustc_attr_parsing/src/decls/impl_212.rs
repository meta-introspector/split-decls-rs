macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        OnDuplicate!();
        Stage!();
        AllowedTargets!();
        DenyExplicitImplParser!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for DenyExplicitImplParser { const PATH : & [Symbol] = & [sym :: rustc_deny_explicit_impl] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: DenyExplicitImpl ; }
    };
}

impl_212!()