// Generated macro for impl_152 (impl)
macro_rules! Depcrate_attributes_dummyimpl_152 {
() => {
// Module: crate::attributes::dummy
// Provides: {"impl_152"}
// Dependencies: {}
impl < S : Stage > SingleAttributeParser < S > for DummyParser { const PATH : & [Symbol] = & [sym :: rustc_dummy] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (Word) ; fn convert (_ : & mut AcceptContext < '_ , '_ , S > , _ : & ArgParser < '_ >) -> Option < AttributeKind > { Some (AttributeKind :: Dummy) } }
};
}
