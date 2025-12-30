// Generated macro for impl_197 (impl)
macro_rules! Depcrate_attributes_lint_helpersimpl_197 {
() => {
// Module: crate::attributes::lint_helpers
// Provides: {"impl_197"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for PubTransparentParser { const PATH : & [Symbol] = & [sym :: rustc_pub_transparent] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Enum) , Allow (Target :: Union) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: PubTransparent ; }
};
}
