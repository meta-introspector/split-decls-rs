// Generated macro for impl_119 (impl)
macro_rules! Depcrate_marshalledimpl_119 {
() => {
// Module: crate::marshalled
// Provides: {"impl_119"}
// Dependencies: {}
impl Marshal for SignatureSingle { fn signature (& self) -> & SignatureSingle { SignatureSingle :: new_unchecked ("g") } fn append_data_to (& self , v : & mut Vec < u8 >) { v . push (self . len () as u8) ; v . extend_from_slice (self . as_bytes ()) ; v . push (0) ; } }
};
}
