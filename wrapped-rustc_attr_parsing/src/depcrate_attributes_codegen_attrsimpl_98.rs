// Generated macro for impl_98 (impl)
macro_rules! Depcrate_attributes_codegen_attrsimpl_98 {
() => {
// Module: crate::attributes::codegen_attrs
// Provides: {"impl_98"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for TrackCallerParser { const PATH : & [Symbol] = & [sym :: track_caller] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: ForeignFn) , Allow (Target :: Closure) , Warn (Target :: MacroDef) , Warn (Target :: Arm) , Warn (Target :: Field) , Warn (Target :: MacroCall) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TrackCaller ; }
};
}
