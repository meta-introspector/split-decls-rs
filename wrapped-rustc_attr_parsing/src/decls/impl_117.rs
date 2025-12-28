macro_rules! deps {
    () => {
        AllowedTargets!();
        OnDuplicate!();
        Stage!();
        NoArgsAttributeParser!();
        ConstContinueParser!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ConstContinueParser { const PATH : & [Symbol] = & [sym :: const_continue] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstContinue ; }
    };
}

impl_117!();