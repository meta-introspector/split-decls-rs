// Generated macro for EmitState (struct)
macro_rules! Depcrate_isa_pulley_shared_inst_emitEmitState {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"EmitState"}
// Dependencies: {}
# [doc = " State carried between emissions of a sequence of instructions."] # [derive (Default , Clone , Debug)] pub struct EmitState < P > where P : PulleyTargetKind , { _phantom : PhantomData < P > , ctrl_plane : ControlPlane , user_stack_map : Option < ir :: UserStackMap > , frame_layout : FrameLayout , }
};
}
