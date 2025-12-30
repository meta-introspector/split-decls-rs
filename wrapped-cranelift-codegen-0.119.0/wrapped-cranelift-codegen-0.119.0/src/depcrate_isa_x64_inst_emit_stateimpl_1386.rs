// Generated macro for impl_1386 (impl)
macro_rules! Depcrate_isa_x64_inst_emit_stateimpl_1386 {
() => {
// Module: crate::isa::x64::inst::emit_state
// Provides: {"impl_1386"}
// Dependencies: {}
impl MachInstEmitState < Inst > for EmitState { fn new (abi : & Callee < X64ABIMachineSpec > , ctrl_plane : ControlPlane) -> Self { EmitState { user_stack_map : None , ctrl_plane , frame_layout : abi . frame_layout () . clone () , } } fn pre_safepoint (& mut self , user_stack_map : Option < ir :: UserStackMap >) { self . user_stack_map = user_stack_map ; } fn ctrl_plane_mut (& mut self) -> & mut ControlPlane { & mut self . ctrl_plane } fn take_ctrl_plane (self) -> ControlPlane { self . ctrl_plane } fn frame_layout (& self) -> & FrameLayout { & self . frame_layout } }
};
}
