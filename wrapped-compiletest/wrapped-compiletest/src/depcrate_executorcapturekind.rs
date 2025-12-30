// Generated macro for CaptureKind (enum)
macro_rules! Depcrate_executorCaptureKind {
() => {
// Module: crate::executor
// Provides: {"CaptureKind"}
// Dependencies: {}
enum CaptureKind { # [doc = " Do not capture test-runner output, for `--no-capture`."] # [doc = ""] # [doc = " (This does not affect `rustc` and other subprocesses spawned by test"] # [doc = " runners, whose output is always captured.)"] None , # [doc = " Use the old output-capture implementation, which relies on the unstable"] # [doc = " library feature `#![feature(internal_output_capture)]`."] Old { buf : Arc < Mutex < Vec < u8 > > > } , # [doc = " Use the new output-capture implementation, which only uses stable Rust."] New { buf : output_capture :: CaptureBuf } , }
};
}
