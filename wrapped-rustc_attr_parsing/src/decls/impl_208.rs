macro_rules! deps {
    () => {
        OnDuplicate!();
        NoArgsAttributeParser!();
        TypeConstParser!();
        Stage!();
        AllowedTargets!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for TypeConstParser { const PATH : & [Symbol] = & [sym :: type_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: AssocConst)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TypeConst ; }
    };
}

impl_208!();