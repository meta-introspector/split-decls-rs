// Generated macro for impl_117 (impl)
macro_rules! Depcrate_marshalledimpl_117 {
() => {
// Module: crate::marshalled
// Provides: {"impl_117"}
// Dependencies: {}
impl Marshal for dbus_strings :: ObjectPath { fn signature (& self) -> & SignatureSingle { SignatureSingle :: new_unchecked ("o") } fn append_data_to (& self , v : & mut Vec < u8 >) { self . as_dbus_str () . append_data_to (v) ; } }
};
}
