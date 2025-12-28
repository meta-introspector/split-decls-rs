macro_rules! deps {
    () => {
        Stage!();
        FfiConstParser!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        AllowedTargets!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for FfiConstParser { const PATH : & [Symbol] = & [sym :: ffi_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: ForeignFn)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: FfiConst ; }
    };
}

impl_95!()