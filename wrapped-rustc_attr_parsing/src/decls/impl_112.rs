macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        AllowedTargets!();
        OnDuplicate!();
        Stage!();
        AutomaticallyDerivedParser!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for AutomaticallyDerivedParser { const PATH : & [Symbol] = & [sym :: automatically_derived] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Impl { of_trait : true }) , Error (Target :: Crate) , Error (Target :: WherePredicate) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AutomaticallyDerived ; }
    };
}

impl_112!();