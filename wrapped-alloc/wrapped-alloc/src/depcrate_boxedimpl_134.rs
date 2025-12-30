// Generated macro for impl_134 (impl)
macro_rules! Depcrate_boxedimpl_134 {
() => {
// Module: crate::boxed
// Provides: {"impl_134"}
// Dependencies: {}
# [stable (feature = "indirect_hasher_impl" , since = "1.22.0")] impl < T : ? Sized + Hasher , A : Allocator > Hasher for Box < T , A > { fn finish (& self) -> u64 { (* * self) . finish () } fn write (& mut self , bytes : & [u8]) { (* * self) . write (bytes) } fn write_u8 (& mut self , i : u8) { (* * self) . write_u8 (i) } fn write_u16 (& mut self , i : u16) { (* * self) . write_u16 (i) } fn write_u32 (& mut self , i : u32) { (* * self) . write_u32 (i) } fn write_u64 (& mut self , i : u64) { (* * self) . write_u64 (i) } fn write_u128 (& mut self , i : u128) { (* * self) . write_u128 (i) } fn write_usize (& mut self , i : usize) { (* * self) . write_usize (i) } fn write_i8 (& mut self , i : i8) { (* * self) . write_i8 (i) } fn write_i16 (& mut self , i : i16) { (* * self) . write_i16 (i) } fn write_i32 (& mut self , i : i32) { (* * self) . write_i32 (i) } fn write_i64 (& mut self , i : i64) { (* * self) . write_i64 (i) } fn write_i128 (& mut self , i : i128) { (* * self) . write_i128 (i) } fn write_isize (& mut self , i : isize) { (* * self) . write_isize (i) } fn write_length_prefix (& mut self , len : usize) { (* * self) . write_length_prefix (len) } fn write_str (& mut self , s : & str) { (* * self) . write_str (s) } }
};
}
