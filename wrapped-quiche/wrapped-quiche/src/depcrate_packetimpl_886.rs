// Generated macro for impl_886 (impl)
macro_rules! Depcrate_packetimpl_886 {
() => {
// Module: crate::packet
// Provides: {"impl_886"}
// Dependencies: {}
impl < T > Index < Epoch > for [T] where T : Sized , { type Output = T ; fn index (& self , index : Epoch) -> & Self :: Output { self . index (usize :: from (index)) } }
};
}
