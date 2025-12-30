// Generated macro for impl_808 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_808 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_808"}
// Dependencies: {}
impl < T : ? Sized > hash :: Hasher for Retained < T > where for < 'a > & 'a T : hash :: Hasher , { fn finish (& self) -> u64 { (& * * self) . finish () } fn write (& mut self , bytes : & [u8]) { (& * * self) . write (bytes) ; } fn write_u8 (& mut self , i : u8) { (& * * self) . write_u8 (i) ; } fn write_u16 (& mut self , i : u16) { (& * * self) . write_u16 (i) ; } fn write_u32 (& mut self , i : u32) { (& * * self) . write_u32 (i) ; } fn write_u64 (& mut self , i : u64) { (& * * self) . write_u64 (i) ; } fn write_u128 (& mut self , i : u128) { (& * * self) . write_u128 (i) ; } fn write_usize (& mut self , i : usize) { (& * * self) . write_usize (i) ; } fn write_i8 (& mut self , i : i8) { (& * * self) . write_i8 (i) ; } fn write_i16 (& mut self , i : i16) { (& * * self) . write_i16 (i) ; } fn write_i32 (& mut self , i : i32) { (& * * self) . write_i32 (i) ; } fn write_i64 (& mut self , i : i64) { (& * * self) . write_i64 (i) ; } fn write_i128 (& mut self , i : i128) { (& * * self) . write_i128 (i) ; } fn write_isize (& mut self , i : isize) { (& * * self) . write_isize (i) ; } }
};
}
