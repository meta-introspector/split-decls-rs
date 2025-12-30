// Generated macro for impl_2436 (impl)
macro_rules! Depcrate_isa_s390x_inst_emitimpl_2436 {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"impl_2436"}
// Dependencies: {}
impl MachInstEmitState < Inst > for EmitState { fn new (abi : & Callee < S390xMachineDeps > , ctrl_plane : ControlPlane) -> Self { EmitState { nominal_sp_offset : 0 , user_stack_map : None , ctrl_plane , frame_layout : abi . frame_layout () . clone () , } } fn pre_safepoint (& mut self , user_stack_map : Option < ir :: UserStackMap >) { self . user_stack_map = user_stack_map ; } fn ctrl_plane_mut (& mut self) -> & mut ControlPlane { & mut self . ctrl_plane } fn take_ctrl_plane (self) -> ControlPlane { self . ctrl_plane } fn frame_layout (& self) -> & FrameLayout { & self . frame_layout } }
};
}
