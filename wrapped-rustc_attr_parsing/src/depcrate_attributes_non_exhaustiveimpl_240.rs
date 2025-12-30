// Generated macro for impl_240 (impl)
macro_rules! Depcrate_attributes_non_exhaustiveimpl_240 {
() => {
// Module: crate::attributes::non_exhaustive
// Provides: {"impl_240"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for NonExhaustiveParser { const PATH : & [Symbol] = & [sym :: non_exhaustive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Enum) , Allow (Target :: Struct) , Allow (Target :: Variant) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NonExhaustive ; }
};
}
