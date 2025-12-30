// Generated macro for impl_181 (impl)
macro_rules! Depcrate_attributes_link_attrsimpl_181 {
() => {
// Module: crate::attributes::link_attrs
// Provides: {"impl_181"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ExportStableParser { const PATH : & [Symbol] = & [sym :: export_stable] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: ExportStable ; }
};
}
