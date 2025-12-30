// Generated macro for EmitState (struct)
macro_rules! Depcrate_isa_riscv64_inst_emitEmitState {
() => {
// Module: crate::isa::riscv64::inst::emit
// Provides: {"EmitState"}
// Dependencies: {}
# [doc = " State carried between emissions of a sequence of instructions."] # [derive (Default , Clone , Debug)] pub struct EmitState { # [doc = " The user stack map for the upcoming instruction, as provided to"] # [doc = " `pre_safepoint()`."] user_stack_map : Option < ir :: UserStackMap > , # [doc = " Only used during fuzz-testing. Otherwise, it is a zero-sized struct and"] # [doc = " optimized away at compiletime. See [cranelift_control]."] ctrl_plane : ControlPlane , # [doc = " Vector State"] # [doc = " Controls the current state of the vector unit at the emission point."] vstate : EmitVState , frame_layout : FrameLayout , }
};
}
