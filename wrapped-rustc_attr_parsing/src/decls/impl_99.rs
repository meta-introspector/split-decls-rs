macro_rules! deps {
    () => {
        NoArgsAttributeParser!();
        Stage!();
        OnDuplicate!();
        AllowedTargets!();
        StdInternalSymbolParser!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for StdInternalSymbolParser { const PATH : & [Symbol] = & [sym :: rustc_std_internal_symbol] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: ForeignFn) , Allow (Target :: Static) , Allow (Target :: ForeignStatic) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: StdInternalSymbol ; }
    };
}

impl_99!();