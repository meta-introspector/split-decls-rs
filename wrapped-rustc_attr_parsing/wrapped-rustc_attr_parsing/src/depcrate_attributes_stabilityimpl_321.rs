// Generated macro for impl_321 (impl)
macro_rules! Depcrate_attributes_stabilityimpl_321 {
() => {
// Module: crate::attributes::stability
// Provides: {"impl_321"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for ConstStabilityIndirectParser { const PATH : & [Symbol] = & [sym :: rustc_const_stable_indirect] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) ,]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: ConstStabilityIndirect ; }
};
}
