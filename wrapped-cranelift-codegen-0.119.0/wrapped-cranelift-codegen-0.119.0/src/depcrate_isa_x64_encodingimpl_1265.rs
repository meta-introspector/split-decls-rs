// Generated macro for impl_1265 (impl)
macro_rules! Depcrate_isa_x64_encodingimpl_1265 {
() => {
// Module: crate::isa::x64::encoding
// Provides: {"impl_1265"}
// Dependencies: {}
# [doc = " Provide a convenient implementation for testing."] impl ByteSink for Vec < u8 > { fn put1 (& mut self , v : u8) { self . extend_from_slice (& [v]) } fn put2 (& mut self , v : u16) { self . extend_from_slice (& v . to_le_bytes ()) } fn put4 (& mut self , v : u32) { self . extend_from_slice (& v . to_le_bytes ()) } fn put8 (& mut self , v : u64) { self . extend_from_slice (& v . to_le_bytes ()) } }
};
}
