macro_rules! deps {
    () => {
        ProcMacroAttributeParser!();
        NoArgsAttributeParser!();
        OnDuplicate!();
        AllowedTargets!();
        Stage!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for ProcMacroAttributeParser { const PATH : & [Symbol] = & [sym :: proc_macro_attribute] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = PROC_MACRO_ALLOWED_TARGETS ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ProcMacroAttribute ; }
    };
}

impl_144!()