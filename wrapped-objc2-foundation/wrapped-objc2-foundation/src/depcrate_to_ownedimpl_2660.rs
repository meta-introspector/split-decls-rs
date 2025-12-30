// Generated macro for impl_2660 (impl)
macro_rules! Depcrate_to_ownedimpl_2660 {
() => {
// Module: crate::to_owned
// Provides: {"impl_2660"}
// Dependencies: {}
# [cfg (feature = "NSArray")] impl < ObjectType : Message > ToOwned for crate :: NSMutableArray < ObjectType > { type Owned = Retained < Self > ; fn to_owned (& self) -> Self :: Owned { self . mutableCopy () } }
};
}
