// Generated macro for impl_360 (impl)
macro_rules! Depcrate_attributes_traitsimpl_360 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_360"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for UnsafeSpecializationMarkerParser { const PATH : & [Symbol] = & [sym :: rustc_unsafe_specialization_marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: UnsafeSpecializationMarker ; }
};
}
