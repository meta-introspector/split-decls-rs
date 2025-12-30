// Generated macro for impl_806 (impl)
macro_rules! Depcrate_rawimpl_806 {
() => {
// Module: crate::raw
// Provides: {"impl_806"}
// Dependencies: {}
impl ToOwned for RawValue { type Owned = Box < RawValue > ; fn to_owned (& self) -> Self :: Owned { RawValue :: from_owned (self . json . to_owned () . into_boxed_str ()) } }
};
}
