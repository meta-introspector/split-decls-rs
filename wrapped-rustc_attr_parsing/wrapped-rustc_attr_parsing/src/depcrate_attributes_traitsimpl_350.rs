// Generated macro for impl_350 (impl)
macro_rules! Depcrate_attributes_traitsimpl_350 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_350"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for MarkerParser { const PATH : & [Symbol] = & [sym :: marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Marker ; }
};
}
