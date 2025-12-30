// Generated macro for impl_2665 (impl)
macro_rules! Depcrate_to_ownedimpl_2665 {
() => {
// Module: crate::to_owned
// Provides: {"impl_2665"}
// Dependencies: {}
# [cfg (feature = "NSSet")] impl < ObjectType : Message > ToOwned for crate :: NSMutableSet < ObjectType > { type Owned = Retained < Self > ; fn to_owned (& self) -> Self :: Owned { self . mutableCopy () } }
};
}
