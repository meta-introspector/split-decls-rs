// Generated macro for impl_37 (impl)
macro_rules! Depcrate_attributes_bodyimpl_37 {
() => {
// Module: crate::attributes::body
// Provides: {"impl_37"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for CoroutineParser { const PATH : & [Symbol] = & [sym :: coroutine] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Closure)]) ; const CREATE : fn (rustc_span :: Span) -> AttributeKind = | span | AttributeKind :: Coroutine (span) ; }
};
}
