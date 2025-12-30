// Generated macro for utils (module)
macro_rules! Depcrateutils {
() => {
// Module: crate
// Provides: {"utils"}
// Dependencies: {}
pub mod utils { # ! [doc = " Miscellaneous utilities."] # ! [doc = ""] # ! [doc = " * [`Backoff`], for exponential backoff in spin loops."] # ! [doc = " * [`CachePadded`], for padding and aligning a value to the length of a cache line."] pub use crossbeam_utils :: Backoff ; pub use crossbeam_utils :: CachePadded ; }
};
}
