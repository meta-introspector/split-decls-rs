// Generated macro for impl_2600 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_emitimpl_2600 {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"impl_2600"}
// Dependencies: {}
impl < P > MachInstEmitState < InstAndKind < P > > for EmitState < P > where P : PulleyTargetKind , { fn new (abi : & Callee < PulleyMachineDeps < P > > , ctrl_plane : ControlPlane) -> Self { EmitState { _phantom : PhantomData , ctrl_plane , user_stack_map : None , frame_layout : abi . frame_layout () . clone () , } } fn pre_safepoint (& mut self , user_stack_map : Option < ir :: UserStackMap >) { self . user_stack_map = user_stack_map ; } fn ctrl_plane_mut (& mut self) -> & mut ControlPlane { & mut self . ctrl_plane } fn take_ctrl_plane (self) -> ControlPlane { self . ctrl_plane } fn frame_layout (& self) -> & FrameLayout { & self . frame_layout } }
};
}
