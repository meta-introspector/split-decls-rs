macro_rules! deps {
    () => {
        Stage!();
        AllowedTargets!();
        PointeeParser!();
        NoArgsAttributeParser!();
        OnDuplicate!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for PointeeParser { const PATH : & [Symbol] = & [sym :: pointee] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Pointee ; }
    };
}

impl_230!()