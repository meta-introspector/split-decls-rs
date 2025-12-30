// Generated macro for impl_364 (impl)
macro_rules! Depcrate_attributes_traitsimpl_364 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_364"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for AllowIncoherentImplParser { const PATH : & [Symbol] = & [sym :: rustc_allow_incoherent_impl] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Method (MethodKind :: Inherent))]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AllowIncoherentImpl ; }
};
}
