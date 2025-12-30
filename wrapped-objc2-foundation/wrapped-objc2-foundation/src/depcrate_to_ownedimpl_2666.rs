// Generated macro for impl_2666 (impl)
macro_rules! Depcrate_to_ownedimpl_2666 {
() => {
// Module: crate::to_owned
// Provides: {"impl_2666"}
// Dependencies: {}
# [cfg (feature = "NSString")] impl ToOwned for crate :: NSString { type Owned = Retained < Self > ; fn to_owned (& self) -> Self :: Owned { self . copy () } }
};
}
