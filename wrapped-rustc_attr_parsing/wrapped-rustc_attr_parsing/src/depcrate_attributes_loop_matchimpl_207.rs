// Generated macro for impl_207 (impl)
macro_rules! Depcrate_attributes_loop_matchimpl_207 {
() => {
// Module: crate::attributes::loop_match
// Provides: {"impl_207"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ConstContinueParser { const PATH : & [Symbol] = & [sym :: const_continue] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstContinue ; }
};
}
