// Generated macro for impl_249 (impl)
macro_rules! Depcrate_attributes_proc_macro_attrsimpl_249 {
() => {
// Module: crate::attributes::proc_macro_attrs
// Provides: {"impl_249"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ProcMacroParser { const PATH : & [Symbol] = & [sym :: proc_macro] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = PROC_MACRO_ALLOWED_TARGETS ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ProcMacro ; }
};
}
