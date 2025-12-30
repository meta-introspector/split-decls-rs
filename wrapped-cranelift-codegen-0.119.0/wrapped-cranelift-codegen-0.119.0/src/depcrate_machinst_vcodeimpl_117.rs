// Generated macro for impl_117 (impl)
macro_rules! Depcrate_machinst_vcodeimpl_117 {
() => {
// Module: crate::machinst::vcode
// Provides: {"impl_117"}
// Dependencies: {}
impl ToBackwardsInsnIndex for InsnIndex { fn to_backwards_insn_index (& self , num_insts : usize) -> BackwardsInsnIndex { BackwardsInsnIndex :: new (num_insts - self . index () - 1) } }
};
}
