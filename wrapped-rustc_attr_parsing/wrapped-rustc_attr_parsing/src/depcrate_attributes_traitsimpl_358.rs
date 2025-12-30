// Generated macro for impl_358 (impl)
macro_rules! Depcrate_attributes_traitsimpl_358 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_358"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for SpecializationTraitParser { const PATH : & [Symbol] = & [sym :: rustc_specialization_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: SpecializationTrait ; }
};
}
