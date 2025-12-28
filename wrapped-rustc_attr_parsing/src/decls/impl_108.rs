macro_rules! deps {
    () => {
        AllowedTargets!();
        Stage!();
        OnDuplicate!();
        NoArgsAttributeParser!();
        PubTransparentParser!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for PubTransparentParser { const PATH : & [Symbol] = & [sym :: rustc_pub_transparent] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Enum) , Allow (Target :: Union) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: PubTransparent ; }
    };
}

impl_108!()