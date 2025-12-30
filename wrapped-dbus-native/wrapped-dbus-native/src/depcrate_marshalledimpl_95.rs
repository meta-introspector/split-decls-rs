// Generated macro for impl_95 (impl)
macro_rules! Depcrate_marshalledimpl_95 {
() => {
// Module: crate::marshalled
// Provides: {"impl_95"}
// Dependencies: {}
impl Marshal for StructBuf { fn signature (& self) -> & SignatureSingle { & self . outer_sig } fn append_data_to (& self , v : & mut Vec < u8 >) { align_buf (v , 8) ; v . extend_from_slice (& self . inner . data) } }
};
}
