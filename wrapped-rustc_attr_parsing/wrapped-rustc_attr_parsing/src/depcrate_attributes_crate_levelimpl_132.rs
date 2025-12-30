// Generated macro for impl_132 (impl)
macro_rules! Depcrate_attributes_crate_levelimpl_132 {
() => {
// Module: crate::attributes::crate_level
// Provides: {"impl_132"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for NoCoreParser { const PATH : & [Symbol] = & [sym :: no_core] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NoCore ; const TYPE : AttributeType = AttributeType :: CrateLevel ; }
};
}
