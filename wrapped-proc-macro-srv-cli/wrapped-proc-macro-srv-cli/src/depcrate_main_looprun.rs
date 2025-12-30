// Generated macro for run (function)
macro_rules! Depcrate_main_looprun {
() => {
// Module: crate::main_loop
// Provides: {"run"}
// Dependencies: {}
pub (crate) fn run (format : ProtocolFormat) -> io :: Result < () > { match format { ProtocolFormat :: Json => run_json () , # [cfg (feature = "postcard")] ProtocolFormat :: Postcard => unimplemented ! () , } }
};
}
