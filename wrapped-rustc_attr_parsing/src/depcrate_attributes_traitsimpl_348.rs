// Generated macro for impl_348 (impl)
macro_rules! Depcrate_attributes_traitsimpl_348 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_348"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for TypeConstParser { const PATH : & [Symbol] = & [sym :: type_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: AssocConst)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TypeConst ; }
};
}
