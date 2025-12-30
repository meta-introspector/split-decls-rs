// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < T > IndexMut < Idx < T > > for Arena < T > { fn index_mut (& mut self , idx : Idx < T >) -> & mut T { let idx = idx . into_raw () . 0 as usize ; & mut self . data [idx] } }
};
}
