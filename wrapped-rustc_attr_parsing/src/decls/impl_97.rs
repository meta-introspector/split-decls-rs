macro_rules! deps {
    () => {
        Stage!();
        AllowedTargets!();
        FfiPureParser!();
        NoArgsAttributeParser!();
        OnDuplicate!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for FfiPureParser { const PATH : & [Symbol] = & [sym :: ffi_pure] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: ForeignFn)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: FfiPure ; }
    };
}

impl_97!()