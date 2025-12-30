// Generated macro for impl_289 (impl)
macro_rules! Depcrate_attributes_reprimpl_289 {
() => {
// Module: crate::attributes::repr
// Provides: {"impl_289"}
// Dependencies: {}
impl < S : Stage > AttributeParser < S > for AlignParser { const ATTRIBUTES : AcceptMapping < Self , S > = & [(Self :: PATH , Self :: TEMPLATE , Self :: parse)] ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Fn) , Allow (Target :: Method (MethodKind :: Inherent)) , Allow (Target :: Method (MethodKind :: Trait { body : true })) , Allow (Target :: Method (MethodKind :: TraitImpl)) , Allow (Target :: Method (MethodKind :: Trait { body : false })) , Allow (Target :: ForeignFn) ,]) ; fn finalize (self , _cx : & FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > { let (align , span) = self . 0 ? ; Some (AttributeKind :: Align { align , span }) } }
};
}
