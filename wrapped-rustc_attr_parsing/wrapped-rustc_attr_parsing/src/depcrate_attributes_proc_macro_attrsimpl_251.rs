// Generated macro for impl_251 (impl)
macro_rules! Depcrate_attributes_proc_macro_attrsimpl_251 {
() => {
// Module: crate::attributes::proc_macro_attrs
// Provides: {"impl_251"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ProcMacroAttributeParser { const PATH : & [Symbol] = & [sym :: proc_macro_attribute] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = PROC_MACRO_ALLOWED_TARGETS ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ProcMacroAttribute ; }
};
}
