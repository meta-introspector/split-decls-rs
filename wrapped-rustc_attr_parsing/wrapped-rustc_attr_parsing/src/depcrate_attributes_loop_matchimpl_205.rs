// Generated macro for impl_205 (impl)
macro_rules! Depcrate_attributes_loop_matchimpl_205 {
() => {
// Module: crate::attributes::loop_match
// Provides: {"impl_205"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for LoopMatchParser { const PATH : & [Symbol] = & [sym :: loop_match] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: LoopMatch ; }
};
}
