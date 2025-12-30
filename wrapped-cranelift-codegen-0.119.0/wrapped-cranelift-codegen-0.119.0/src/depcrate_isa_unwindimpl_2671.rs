// Generated macro for impl_2671 (impl)
macro_rules! Depcrate_isa_unwindimpl_2671 {
() => {
// Module: crate::isa::unwind
// Provides: {"impl_2671"}
// Dependencies: {}
impl < 'a > Writer < 'a > { pub fn new (buf : & 'a mut [u8]) -> Self { Self { buf , offset : 0 } } fn write_u8 (& mut self , v : u8) { self . buf [self . offset] = v ; self . offset += 1 ; } fn write_u16_le (& mut self , v : u16) { self . buf [self . offset .. (self . offset + 2)] . copy_from_slice (& v . to_le_bytes ()) ; self . offset += 2 ; } fn write_u16_be (& mut self , v : u16) { self . buf [self . offset .. (self . offset + 2)] . copy_from_slice (& v . to_be_bytes ()) ; self . offset += 2 ; } fn write_u32_le (& mut self , v : u32) { self . buf [self . offset .. (self . offset + 4)] . copy_from_slice (& v . to_le_bytes ()) ; self . offset += 4 ; } fn write_u32_be (& mut self , v : u32) { self . buf [self . offset .. (self . offset + 4)] . copy_from_slice (& v . to_be_bytes ()) ; self . offset += 4 ; } }
};
}
