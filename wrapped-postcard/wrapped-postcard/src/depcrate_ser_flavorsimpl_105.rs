// Generated macro for impl_105 (impl)
macro_rules! Depcrate_ser_flavorsimpl_105 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_105"}
// Dependencies: {}
impl IndexMut < usize > for Slice < '_ > { fn index_mut (& mut self , idx : usize) -> & mut u8 { let len = (self . end as usize) - (self . start as usize) ; assert ! (idx < len) ; unsafe { & mut * self . start . add (idx) } } }
};
}
