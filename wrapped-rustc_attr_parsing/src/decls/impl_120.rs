macro_rules! deps {
    () => {
        MacroEscapeParser!();
        OnDuplicate!();
        Stage!();
        NoArgsAttributeParser!();
        AllowedTargets!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for MacroEscapeParser { const PATH : & [Symbol] = & [sym :: macro_escape] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = MACRO_USE_ALLOWED_TARGETS ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: MacroEscape ; }
    };
}

impl_120!();