// Generated macro for impl_94 (impl)
macro_rules! Depcrate_machinst_lowerimpl_94 {
() => {
// Module: crate::machinst::lower
// Provides: {"impl_94"}
// Dependencies: {}
# [doc = " Function-level queries."] impl < 'func , I : VCodeInst > Lower < 'func , I > { pub fn dfg (& self) -> & DataFlowGraph { & self . f . dfg } # [doc = " Get the `Callee`."] pub fn abi (& self) -> & Callee < I :: ABIMachineSpec > { self . vcode . abi () } # [doc = " Get the `Callee`."] pub fn abi_mut (& mut self) -> & mut Callee < I :: ABIMachineSpec > { self . vcode . abi_mut () } }
};
}
