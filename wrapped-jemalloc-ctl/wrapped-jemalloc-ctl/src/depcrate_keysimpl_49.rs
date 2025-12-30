// Generated macro for impl_49 (impl)
macro_rules! Depcrate_keysimpl_49 {
() => {
// Module: crate::keys
// Provides: {"impl_49"}
// Dependencies: {}
impl < T : MibArg > ops :: IndexMut < usize > for MibStr < T > { fn index_mut (& mut self , idx : usize) -> & mut Self :: Output { & mut self . 0 . as_mut () [idx] } }
};
}
