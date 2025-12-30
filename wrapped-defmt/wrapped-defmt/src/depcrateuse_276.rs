// Generated macro for use_276 (pub_use)
macro_rules! Depcrateuse_276 {
() => {
// Module: crate
// Provides: {"use_276"}
// Dependencies: {}
# [doc = " Defines the global timestamp provider for defmt."] # [doc = ""] # [doc = " This macro can be used to attach a timestamp or other data to every defmt message. Its syntax"] # [doc = " works exactly like the logging macros, except that no local variables can be accessed and the"] # [doc = " macro should be placed in a module instead of a function."] # [doc = ""] # [doc = " `timestamp!` must only be used once across the crate graph."] # [doc = ""] # [doc = " If no crate defines a timestamp, no timestamp will be included in the logged messages."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use core::sync::atomic::{AtomicU32, Ordering};"] # [doc = ""] # [doc = " static COUNT: AtomicU32 = AtomicU32::new(0);"] # [doc = " defmt::timestamp!(\"{=u32:us}\", COUNT.fetch_add(1, Ordering::Relaxed));"] # [doc = " ```"] pub use defmt_macros :: timestamp ;
};
}
