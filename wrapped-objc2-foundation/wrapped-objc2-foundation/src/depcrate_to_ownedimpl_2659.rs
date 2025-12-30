// Generated macro for impl_2659 (impl)
macro_rules! Depcrate_to_ownedimpl_2659 {
() => {
// Module: crate::to_owned
// Provides: {"impl_2659"}
// Dependencies: {}
# [cfg (feature = "NSArray")] impl < ObjectType : Message > ToOwned for crate :: NSArray < ObjectType > { type Owned = Retained < Self > ; fn to_owned (& self) -> Self :: Owned { self . copy () } }
};
}
