macro_rules! deps {
    () => {
        Stage!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        NoStdParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for NoStdParser { const PATH : & [Symbol] = & [sym :: no_std] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoStd ; const TYPE : AttributeType = AttributeType :: CrateLevel ; }
    };
}

impl_71!();