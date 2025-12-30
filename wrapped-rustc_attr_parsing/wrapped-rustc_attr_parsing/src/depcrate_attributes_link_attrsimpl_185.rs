// Generated macro for impl_185 (impl)
macro_rules! Depcrate_attributes_link_attrsimpl_185 {
() => {
// Module: crate::attributes::link_attrs
// Provides: {"impl_185"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for FfiPureParser { const PATH : & [Symbol] = & [sym :: ffi_pure] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: ForeignFn)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: FfiPure ; }
};
}
