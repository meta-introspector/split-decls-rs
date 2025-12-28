macro_rules! deps {
    () => {
        Stage!();
        OnDuplicate!();
        AllowedTargets!();
        NoArgsAttributeParser!();
        AllowIncoherentImplParser!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for AllowIncoherentImplParser { const PATH : & [Symbol] = & [sym :: rustc_allow_incoherent_impl] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Method (MethodKind :: Inherent))]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AllowIncoherentImpl ; }
    };
}

impl_224!();