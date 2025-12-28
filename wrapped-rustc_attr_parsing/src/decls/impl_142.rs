macro_rules! deps {
    () => {
        AllowedTargets!();
        OnDuplicate!();
        ProcMacroParser!();
        NoArgsAttributeParser!();
        Stage!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ProcMacroParser { const PATH : & [Symbol] = & [sym :: proc_macro] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = PROC_MACRO_ALLOWED_TARGETS ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ProcMacro ; }
    };
}

impl_142!()