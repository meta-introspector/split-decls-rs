// Generated macro for impl_230 (impl)
macro_rules! Depcrate_attributes_no_implicit_preludeimpl_230 {
() => {
// Module: crate::attributes::no_implicit_prelude
// Provides: {"impl_230"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for NoImplicitPreludeParser { const PATH : & [rustc_span :: Symbol] = & [sym :: no_implicit_prelude] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Mod) , Allow (Target :: Crate)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoImplicitPrelude ; }
};
}
