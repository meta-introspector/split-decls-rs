// Generated macro for impl_201 (impl)
macro_rules! Depcrate_attributes_lint_helpersimpl_201 {
() => {
// Module: crate::attributes::lint_helpers
// Provides: {"impl_201"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for AutomaticallyDerivedParser { const PATH : & [Symbol] = & [sym :: automatically_derived] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Impl { of_trait : true }) , Error (Target :: Crate) , Error (Target :: WherePredicate) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AutomaticallyDerived ; }
};
}
