// Generated macro for impl_199 (impl)
macro_rules! Depcrate_attributes_lint_helpersimpl_199 {
() => {
// Module: crate::attributes::lint_helpers
// Provides: {"impl_199"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for PassByValueParser { const PATH : & [Symbol] = & [sym :: rustc_pass_by_value] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Enum) , Allow (Target :: TyAlias) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: PassByValue ; }
};
}
