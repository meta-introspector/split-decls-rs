// Generated macro for impl_370 (impl)
macro_rules! Depcrate_attributes_traitsimpl_370 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_370"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for PointeeParser { const PATH : & [Symbol] = & [sym :: pointee] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Pointee ; }
};
}
