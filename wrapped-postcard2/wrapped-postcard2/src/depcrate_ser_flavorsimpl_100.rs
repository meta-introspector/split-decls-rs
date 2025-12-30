// Generated macro for impl_100 (impl)
macro_rules! Depcrate_ser_flavorsimpl_100 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_100"}
// Dependencies: {}
impl IndexMut < usize > for Slice < '_ > { fn index_mut (& mut self , idx : usize) -> & mut u8 { let len = (self . end as usize) - (self . start as usize) ; assert ! (idx < len) ; unsafe { & mut * self . start . add (idx) } } }
};
}
