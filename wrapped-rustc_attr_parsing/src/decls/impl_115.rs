macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        AllowedTargets!();
        OnDuplicate!();
        Stage!();
        LoopMatchParser!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for LoopMatchParser { const PATH : & [Symbol] = & [sym :: loop_match] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: LoopMatch ; }
    };
}

impl_115!()