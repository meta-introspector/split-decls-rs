// Generated macro for impl_275 (impl)
macro_rules! Depcrate_value_rawimpl_275 {
() => {
// Module: crate::value::raw
// Provides: {"impl_275"}
// Dependencies: {}
impl ToOwned for RawValue { type Owned = Box < RawValue > ; fn to_owned (& self) -> Self :: Owned { RawValue :: from_boxed_str (self . ron . to_owned () . into_boxed_str ()) } }
};
}
