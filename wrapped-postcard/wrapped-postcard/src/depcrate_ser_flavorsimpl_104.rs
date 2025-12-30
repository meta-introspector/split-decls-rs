// Generated macro for impl_104 (impl)
macro_rules! Depcrate_ser_flavorsimpl_104 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_104"}
// Dependencies: {}
impl Index < usize > for Slice < '_ > { type Output = u8 ; fn index (& self , idx : usize) -> & u8 { let len = (self . end as usize) - (self . start as usize) ; assert ! (idx < len) ; unsafe { & * self . start . add (idx) } } }
};
}
