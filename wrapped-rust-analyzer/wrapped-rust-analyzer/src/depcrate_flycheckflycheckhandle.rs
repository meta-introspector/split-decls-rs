// Generated macro for FlycheckHandle (struct)
macro_rules! Depcrate_flycheckFlycheckHandle {
() => {
// Module: crate::flycheck
// Provides: {"FlycheckHandle"}
// Dependencies: {}
# [doc = " Flycheck wraps the shared state and communication machinery used for"] # [doc = " running `cargo check` (or other compatible command) and providing"] # [doc = " diagnostics based on the output."] # [doc = " The spawned thread is shut down when this struct is dropped."] # [derive (Debug)] pub (crate) struct FlycheckHandle { sender : Sender < StateChange > , _thread : stdx :: thread :: JoinHandle , id : usize , generation : AtomicUsize , }
};
}
