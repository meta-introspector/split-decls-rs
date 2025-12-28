macro_rules! deps {
    () => {
        OnDuplicate!();
        MarkerParser!();
        AllowedTargets!();
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for MarkerParser { const PATH : & [Symbol] = & [sym :: marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Marker ; }
    };
}

impl_210!()