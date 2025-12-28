macro_rules! deps {
    () => {
        OnDuplicate!();
        CoroutineParser!();
        AllowedTargets!();
        Stage!();
        NoArgsAttributeParser!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < S : Stage > NoArgsAttributeParser < S > for CoroutineParser { const PATH : & [Symbol] = & [sym :: coroutine] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Closure)]) ; const CREATE : fn (rustc_span :: Span) -> AttributeKind = | span | AttributeKind :: Coroutine (span) ; }
    };
}

impl_10!()