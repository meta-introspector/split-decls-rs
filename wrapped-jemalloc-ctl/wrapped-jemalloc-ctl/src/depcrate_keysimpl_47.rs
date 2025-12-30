// Generated macro for impl_47 (impl)
macro_rules! Depcrate_keysimpl_47 {
() => {
// Module: crate::keys
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : MibArg > ops :: IndexMut < usize > for Mib < T > { fn index_mut (& mut self , idx : usize) -> & mut Self :: Output { & mut self . 0 . as_mut () [idx] } }
};
}
