// Generated macro for impl_366 (impl)
macro_rules! Depcrate_attributes_traitsimpl_366 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_366"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for CoherenceIsCoreParser { const PATH : & [Symbol] = & [sym :: rustc_coherence_is_core] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Crate)]) ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: CoherenceIsCore ; }
};
}
