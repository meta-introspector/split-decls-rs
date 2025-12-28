macro_rules! deps {
    () => {
        Stage!();
        OnDuplicate!();
        NoArgsAttributeParser!();
        AllowedTargets!();
        CoherenceIsCoreParser!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for CoherenceIsCoreParser { const PATH : & [Symbol] = & [sym :: rustc_coherence_is_core] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Crate)]) ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: CoherenceIsCore ; }
    };
}

impl_226!();