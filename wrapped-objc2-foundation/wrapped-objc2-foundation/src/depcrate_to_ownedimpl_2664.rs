// Generated macro for impl_2664 (impl)
macro_rules! Depcrate_to_ownedimpl_2664 {
() => {
// Module: crate::to_owned
// Provides: {"impl_2664"}
// Dependencies: {}
# [cfg (feature = "NSSet")] impl < ObjectType : Message > ToOwned for crate :: NSSet < ObjectType > { type Owned = Retained < Self > ; fn to_owned (& self) -> Self :: Owned { self . copy () } }
};
}
