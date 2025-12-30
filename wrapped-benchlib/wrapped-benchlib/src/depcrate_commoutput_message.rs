// Generated macro for output_message (function)
macro_rules! Depcrate_commoutput_message {
() => {
// Module: crate::comm
// Provides: {"output_message"}
// Dependencies: {}
# [doc = " Messages are communicated as line-delimited JSON."] pub fn output_message < W : Write > (mut sink : W , message : BenchmarkMessage) -> anyhow :: Result < () > { serde_json :: to_writer (& mut sink , & message) ? ; sink . write_all (b"\n") ? ; Ok (()) }
};
}
