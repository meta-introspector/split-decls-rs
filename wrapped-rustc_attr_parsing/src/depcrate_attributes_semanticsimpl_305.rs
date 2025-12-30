// Generated macro for impl_305 (impl)
macro_rules! Depcrate_attributes_semanticsimpl_305 {
() => {
// Module: crate::attributes::semantics
// Provides: {"impl_305"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for MayDangleParser { const PATH : & [Symbol] = & [sym :: may_dangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (span : Span) -> AttributeKind = AttributeKind :: MayDangle ; }
};
}
