// Generated macro for impl_195 (impl)
macro_rules! Depcrate_attributes_lint_helpersimpl_195 {
() => {
// Module: crate::attributes::lint_helpers
// Provides: {"impl_195"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for AsPtrParser { const PATH : & [Symbol] = & [sym :: rustc_as_ptr] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AsPtr ; }
};
}
