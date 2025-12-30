// Generated macro for impl_134 (impl)
macro_rules! Depcrate_attributes_crate_levelimpl_134 {
() => {
// Module: crate::attributes::crate_level
// Provides: {"impl_134"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for NoStdParser { const PATH : & [Symbol] = & [sym :: no_std] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoStd ; const TYPE : AttributeType = AttributeType :: CrateLevel ; }
};
}
