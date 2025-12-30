// Generated macro for impl_100 (impl)
macro_rules! Depcrate_attributes_codegen_attrsimpl_100 {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"impl_100"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for NoMangleParser { const PATH : & [Symbol] = & [sym :: no_mangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Allow (Target :: Static) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: TraitImpl)) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoMangle ; }
};
}
