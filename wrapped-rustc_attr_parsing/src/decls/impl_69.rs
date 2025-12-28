macro_rules! deps {
    () => {
        NoCoreParser!();
        OnDuplicate!();
        AllowedTargets!();
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for NoCoreParser { const PATH : & [Symbol] = & [sym :: no_core] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoCore ; const TYPE : AttributeType = AttributeType :: CrateLevel ; }
    };
}

impl_69!();