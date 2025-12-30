// Generated macro for impl_346 (impl)
macro_rules! Depcrate_attributes_traitsimpl_346 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_346"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ParenSugarParser { const PATH : & [Symbol] = & [sym :: rustc_paren_sugar] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ParenSugar ; }
};
}
