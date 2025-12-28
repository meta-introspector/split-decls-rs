macro_rules! deps {
    () => {
        AllowedTargets!();
        Stage!();
        ConstTraitParser!();
        OnDuplicate!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ConstTraitParser { const PATH : & [Symbol] = & [sym :: const_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstTrait ; }
    };
}

impl_216!()