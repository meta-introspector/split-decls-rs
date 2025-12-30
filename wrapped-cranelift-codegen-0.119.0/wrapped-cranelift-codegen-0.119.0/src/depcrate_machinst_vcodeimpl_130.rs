// Generated macro for impl_130 (impl)
macro_rules! Depcrate_machinst_vcodeimpl_130 {
() => {
// Module: crate::machinst::vcode
// Provides: {"impl_130"}
// Dependencies: {}
impl < I : VCodeInst > std :: ops :: Index < InsnIndex > for VCode < I > { type Output = I ; fn index (& self , idx : InsnIndex) -> & Self :: Output { & self . insts [idx . index ()] } }
};
}
