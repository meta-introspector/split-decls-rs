// Generated macro for impl_356 (impl)
macro_rules! Depcrate_attributes_traitsimpl_356 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_356"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ConstTraitParser { const PATH : & [Symbol] = & [sym :: const_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstTrait ; }
};
}
