// Generated macro for impl_368 (impl)
macro_rules! Depcrate_attributes_traitsimpl_368 {
() => {
// Module: crate::attributes::traits
// Provides: {"impl_368"}
// Dependencies: {}
impl < S : Stage > NoArgsAttributeParser < S > for FundamentalParser { const PATH : & [Symbol] = & [sym :: fundamental] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: Fundamental ; }
};
}
