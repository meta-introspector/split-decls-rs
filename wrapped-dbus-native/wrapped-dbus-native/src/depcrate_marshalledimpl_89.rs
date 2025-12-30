// Generated macro for impl_89 (impl)
macro_rules! Depcrate_marshalledimpl_89 {
() => {
// Module: crate::marshalled
// Provides: {"impl_89"}
// Dependencies: {}
impl Marshal for ArrayBuf { fn signature (& self) -> & SignatureSingle { & self . outer_sig } fn append_data_to (& self , v : & mut Vec < u8 >) { let slen = self . data . len () as u32 ; slen . append_data_to (v) ; align_buf (v , align_of (self . outer_sig . as_bytes () [1])) ; v . extend_from_slice (& self . data) ; } }
};
}
