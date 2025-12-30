// Generated macro for LoopInstruction (enum)
macro_rules! Depcrate_firstpassLoopInstruction {
() => {
// Module: crate::firstpass
// Provides: {"LoopInstruction"}
// Dependencies: {}
enum LoopInstruction < T > { # [doc = " Continue looking for more special bytes, but skip next few bytes."] ContinueAndSkip (usize) , # [doc = " Break looping immediately, returning with the given index and value."] BreakAtWith (usize , T) , }
};
}
