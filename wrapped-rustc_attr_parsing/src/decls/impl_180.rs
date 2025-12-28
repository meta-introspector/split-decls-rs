macro_rules! deps {
    () => {
        Stage!();
        AllowedTargets!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        MayDangleParser!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for MayDangleParser { const PATH : & [Symbol] = & [sym :: may_dangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (span : Span) -> AttributeKind = AttributeKind :: MayDangle ; }
    };
}

impl_180!();