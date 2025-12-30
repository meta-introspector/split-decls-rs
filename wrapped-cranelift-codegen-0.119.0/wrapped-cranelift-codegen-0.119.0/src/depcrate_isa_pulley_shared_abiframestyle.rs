// Generated macro for FrameStyle (enum)
macro_rules! Depcrate_isa_pulley_shared_abiFrameStyle {
() => {
// Module: crate::isa::pulley_shared::abi
// Provides: {"FrameStyle"}
// Dependencies: {}
# [doc = " Different styles of management of fp/lr and clobbered registers."] # [doc = ""] # [doc = " This helps decide, depending on Cranelift settings and frame layout, what"] # [doc = " macro instruction is used to setup the pulley frame."] enum FrameStyle { # [doc = " No management is happening, fp/lr aren't saved by Pulley or Cranelift."] # [doc = " No stack is being allocated either."] None , # [doc = " Pulley saves the fp/lr combo and then stack adjustments/clobbers are"] # [doc = " handled manually."] PulleyBasicSetup { frame_size : u32 } , # [doc = " Pulley is managing the fp/lr combo, the stack size, and clobbered"] # [doc = " X-class registers."] # [doc = ""] # [doc = " Note that `saved_by_pulley` is not the exhaustive set of clobbered"] # [doc = " registers. It's only those that are part of the `PushFrameSave`"] # [doc = " instruction."] PulleySetupAndSaveClobbers { # [doc = " The size of the frame, including clobbers, that's being allocated."] frame_size : u16 , # [doc = " Registers that pulley is saving/restoring."] saved_by_pulley : ScalarBitSet < u16 > , } , # [doc = " Cranelift is manually managing everything, both clobbers and stack"] # [doc = " increments/decrements."] # [doc = ""] # [doc = " Note that fp/lr are not saved in this mode."] Manual { # [doc = " The size of the stack being allocated."] frame_size : u32 , } , }
};
}
