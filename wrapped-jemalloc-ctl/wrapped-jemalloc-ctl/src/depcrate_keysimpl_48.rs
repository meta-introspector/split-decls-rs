// Generated macro for impl_48 (impl)
macro_rules! Depcrate_keysimpl_48 {
() => {
// Module: crate::keys
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : MibArg > ops :: Index < usize > for MibStr < T > { type Output = usize ; fn index (& self , idx : usize) -> & Self :: Output { & self . 0 . as_ref () [idx] } }
};
}
