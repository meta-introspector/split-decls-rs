// Generated macro for impl_745 (impl)
macro_rules! Depcrate_rawimpl_745 {
() => {
// Module: crate::raw
// Provides: {"impl_745"}
// Dependencies: {}
impl ToOwned for RawValue { type Owned = Box < RawValue > ; fn to_owned (& self) -> Self :: Owned { RawValue :: from_owned (self . json . to_owned () . into_boxed_str ()) } }
};
}
