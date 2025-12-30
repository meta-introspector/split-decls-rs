// Generated macro for impl_90 (impl)
macro_rules! Depcrate_attributes_codegen_attrsimpl_90 {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"impl_90"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ColdParser { const PATH : & [Symbol] = & [sym :: cold] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowListWarnRest (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: ForeignFn) , Allow (Target :: Closure) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Cold ; }
};
}
