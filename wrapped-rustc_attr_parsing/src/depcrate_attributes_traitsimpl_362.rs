// Generated macro for impl_362 (impl)
macro_rules! Depcrate_attributes_traitsimpl_362 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_362"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for CoinductiveParser { const PATH : & [Symbol] = & [sym :: rustc_coinductive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Coinductive ; }
};
}
