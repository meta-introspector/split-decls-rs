// Generated macro for impl_292 (impl)
macro_rules! Depcrate_attributes_reprimpl_292 {
() => {
// Module: crate::attributes::repr
// Provides: {"impl_292"}
// Dependencies: {}
impl < S : Stage > AttributeParser < S > for AlignStaticParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Static) , Allow (Target :: ForeignStatic)]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 . 0 ? ; Some (AttributeKind :: Align { align , span }) } }
};
}
