// Generated macro for impl_220 (impl)
macro_rules! Depcrate_attributes_macro_attrsimpl_220 {
() => {
// Module: crate::attributes::macro_attrs
// Provides: {"impl_220"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for AllowInternalUnsafeParser { const PATH : & [Symbol] = & [sym :: allow_internal_unsafe] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: MacroDef) , Warn (Target :: Field) , Warn (Target :: Arm) ,]) ; const CREATE : fn (Span) -> AttributeKind = | span | AttributeKind :: AllowInternalUnsafe (span) ; }
};
}
