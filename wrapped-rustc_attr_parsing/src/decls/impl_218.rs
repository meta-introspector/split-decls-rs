macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        AllowedTargets!();
        Stage!();
        OnDuplicate!();
        SpecializationTraitParser!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for SpecializationTraitParser { const PATH : & [Symbol] = & [sym :: rustc_specialization_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: SpecializationTrait ; }
    };
}

impl_218!();