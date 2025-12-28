macro_rules! deps {
    () => {
        TypeConstParser!();
        AllowedTargets!();
        OnDuplicate!();
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for TypeConstParser { const PATH : & [Symbol] = & [sym :: type_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: AssocConst)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TypeConst ; }
    };
}

impl_208!()