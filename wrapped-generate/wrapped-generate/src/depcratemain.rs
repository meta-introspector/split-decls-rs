// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> io :: Result < () > { let out_dir = PathBuf :: from (env ! ("OUT_DIR")) ; fs :: copy (out_dir . join ("logproto.rs") , "../src/logproto.rs") ? ; fs :: copy (out_dir . join ("stats.rs") , "../src/stats.rs") ? ; Ok (()) }
};
}
