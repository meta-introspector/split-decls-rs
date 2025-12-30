// Generated macro for EmitState (struct)
macro_rules! Depcrate_isa_s390x_inst_emitEmitState {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"EmitState"}
// Dependencies: {}
# [doc = " State carried between emissions of a sequence of instructions."] # [derive (Default , Clone , Debug)] pub struct EmitState { # [doc = " Offset from the actual SP to the \"nominal SP\".  The latter is defined"] # [doc = " as the value the stack pointer has after the prolog.  This offset is"] # [doc = " normally always zero, except during a call sequence using the tail-call"] # [doc = " ABI, between the AllocateArgs and the actual call instruction."] pub (crate) nominal_sp_offset : u32 , # [doc = " The user stack map for the upcoming instruction, as provided to"] # [doc = " `pre_safepoint()`."] user_stack_map : Option < ir :: UserStackMap > , # [doc = " Only used during fuzz-testing. Otherwise, it is a zero-sized struct and"] # [doc = " optimized away at compiletime. See [cranelift_control]."] ctrl_plane : ControlPlane , frame_layout : FrameLayout , }
};
}
