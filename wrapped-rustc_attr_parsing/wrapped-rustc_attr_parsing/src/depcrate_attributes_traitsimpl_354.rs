// Generated macro for impl_354 (impl)
macro_rules! Depcrate_attributes_traitsimpl_354 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_354"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for DoNotImplementViaObjectParser { const PATH : & [Symbol] = & [sym :: rustc_do_not_implement_via_object] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: DoNotImplementViaObject ; }
};
}
