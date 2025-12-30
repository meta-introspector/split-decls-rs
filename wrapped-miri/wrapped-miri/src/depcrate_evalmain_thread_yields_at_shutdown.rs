// Generated macro for MAIN_THREAD_YIELDS_AT_SHUTDOWN (const)
macro_rules! Depcrate_evalMAIN_THREAD_YIELDS_AT_SHUTDOWN {
() => {
// Module: crate::eval
// Provides: {"MAIN_THREAD_YIELDS_AT_SHUTDOWN"}
// Dependencies: {}
# [doc = " When the main thread would exit, we will yield to any other thread that is ready to execute."] # [doc = " But we must only do that a finite number of times, or a background thread running `loop {}`"] # [doc = " will hang the program."] const MAIN_THREAD_YIELDS_AT_SHUTDOWN : u32 = 256 ;
};
}
