// Generated macro for impl_98 (impl)
macro_rules! Depcrate_marshalledimpl_98 {
() => {
// Module: crate::marshalled
// Provides: {"impl_98"}
// Dependencies: {}
impl Marshal for VariantBuf { fn signature (& self) -> & SignatureSingle { SignatureSingle :: new_unchecked ("v") } fn append_data_to (& self , v : & mut Vec < u8 >) { (& * self . sig) . append_data_to (v) ; align_buf (v , align_of (self . sig . as_bytes () [0])) ; v . extend_from_slice (& self . data) ; } }
};
}
