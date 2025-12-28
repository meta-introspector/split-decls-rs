macro_rules! deps {
    () => {
        Stage!();
        NoImplicitPreludeParser!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        AllowedTargets!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for NoImplicitPreludeParser { const PATH : & [rustc_span :: Symbol] = & [sym :: no_implicit_prelude] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Mod) , Allow (Target :: Crate)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoImplicitPrelude ; }
    };
}

impl_132!();