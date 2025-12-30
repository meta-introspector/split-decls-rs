// Generated macro for impl_183 (impl)
macro_rules! Depcrate_attributes_link_attrsimpl_183 {
() => {
// Module: crate::attributes::link_attrs
// Provides: {"impl_183"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for FfiConstParser { const PATH : & [Symbol] = & [sym :: ffi_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: ForeignFn)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: FfiConst ; }
};
}
