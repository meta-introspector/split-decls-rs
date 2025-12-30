// Generated macro for impl_2102 (impl)
macro_rules! Depcrate_isa_riscv64_inst_emitimpl_2102 {
() => {
// Module: crate::isa::riscv64::inst::emit
// Provides: {"impl_2102"}
// Dependencies: {}
impl MachInstEmitState < Inst > for EmitState { fn new (abi : & Callee < crate :: isa :: riscv64 :: abi :: Riscv64MachineDeps > , ctrl_plane : ControlPlane ,) -> Self { EmitState { user_stack_map : None , ctrl_plane , vstate : EmitVState :: Unknown , frame_layout : abi . frame_layout () . clone () , } } fn pre_safepoint (& mut self , user_stack_map : Option < ir :: UserStackMap >) { self . user_stack_map = user_stack_map ; } fn ctrl_plane_mut (& mut self) -> & mut ControlPlane { & mut self . ctrl_plane } fn take_ctrl_plane (self) -> ControlPlane { self . ctrl_plane } fn on_new_block (& mut self) { self . vstate = EmitVState :: Unknown ; } fn frame_layout (& self) -> & FrameLayout { & self . frame_layout } }
};
}
