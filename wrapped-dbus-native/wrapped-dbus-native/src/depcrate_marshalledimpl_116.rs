// Generated macro for impl_116 (impl)
macro_rules! Depcrate_marshalledimpl_116 {
() => {
// Module: crate::marshalled
// Provides: {"impl_116"}
// Dependencies: {}
impl Marshal for DBusStr { fn signature (& self) -> & SignatureSingle { SignatureSingle :: new_unchecked ("s") } fn append_data_to (& self , v : & mut Vec < u8 >) { let slen = self . len () as u32 ; slen . append_data_to (v) ; v . extend_from_slice (self . as_bytes ()) ; v . push (0) ; } }
};
}
