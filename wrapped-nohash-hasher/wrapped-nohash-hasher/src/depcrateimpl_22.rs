// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (not (debug_assertions))] impl < T : IsEnabled > Hasher for NoHashHasher < T > { fn write (& mut self , _ : & [u8]) { panic ! ("Invalid use of NoHashHasher") } fn write_u8 (& mut self , n : u8) { self . 0 = u64 :: from (n) } fn write_u16 (& mut self , n : u16) { self . 0 = u64 :: from (n) } fn write_u32 (& mut self , n : u32) { self . 0 = u64 :: from (n) } fn write_u64 (& mut self , n : u64) { self . 0 = n } fn write_usize (& mut self , n : usize) { self . 0 = n as u64 } fn write_i8 (& mut self , n : i8) { self . 0 = n as u64 } fn write_i16 (& mut self , n : i16) { self . 0 = n as u64 } fn write_i32 (& mut self , n : i32) { self . 0 = n as u64 } fn write_i64 (& mut self , n : i64) { self . 0 = n as u64 } fn write_isize (& mut self , n : isize) { self . 0 = n as u64 } fn finish (& self) -> u64 { self . 0 } }
};
}
