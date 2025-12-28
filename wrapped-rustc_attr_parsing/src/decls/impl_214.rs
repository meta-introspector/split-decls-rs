macro_rules! deps {
    () => {
        DoNotImplementViaObjectParser!();
        Stage!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        AllowedTargets!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for DoNotImplementViaObjectParser { const PATH : & [Symbol] = & [sym :: rustc_do_not_implement_via_object] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: DoNotImplementViaObject ; }
    };
}

impl_214!();