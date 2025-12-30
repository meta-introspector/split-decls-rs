// Generated macro for impl_46 (impl)
macro_rules! Depcrate_keysimpl_46 {
() => {
// Module: crate::keys
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : MibArg > ops :: Index < usize > for Mib < T > { type Output = usize ; fn index (& self , idx : usize) -> & Self :: Output { & self . 0 . as_ref () [idx] } }
};
}
