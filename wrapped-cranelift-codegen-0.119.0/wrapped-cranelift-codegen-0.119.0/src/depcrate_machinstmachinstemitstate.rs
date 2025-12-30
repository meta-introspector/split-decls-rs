// Generated macro for MachInstEmitState (trait)
macro_rules! Depcrate_machinstMachInstEmitState {
() => {
// Module: crate::machinst
// Provides: {"MachInstEmitState"}
// Dependencies: {}
# [doc = " A trait describing the emission state carried between MachInsts when"] # [doc = " emitting a function body."] pub trait MachInstEmitState < I : VCodeInst > : Default + Clone + Debug { # [doc = " Create a new emission state given the ABI object."] fn new (abi : & Callee < I :: ABIMachineSpec > , ctrl_plane : ControlPlane) -> Self ; # [doc = " Update the emission state before emitting an instruction that is a"] # [doc = " safepoint."] fn pre_safepoint (& mut self , user_stack_map : Option < ir :: UserStackMap >) ; # [doc = " The emission state holds ownership of a control plane, so it doesn't"] # [doc = " have to be passed around explicitly too much. `ctrl_plane_mut` may"] # [doc = " be used if temporary access to the control plane is needed by some"] # [doc = " other function that doesn't have access to the emission state."] fn ctrl_plane_mut (& mut self) -> & mut ControlPlane ; # [doc = " Used to continue using a control plane after the emission state is"] # [doc = " not needed anymore."] fn take_ctrl_plane (self) -> ControlPlane ; # [doc = " A hook that triggers when first emitting a new block."] # [doc = " It is guaranteed to be called before any instructions are emitted."] fn on_new_block (& mut self) { } # [doc = " The [`FrameLayout`] for the function currently being compiled."] fn frame_layout (& self) -> & FrameLayout ; }
};
}
