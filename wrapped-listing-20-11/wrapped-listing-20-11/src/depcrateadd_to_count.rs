// Generated macro for add_to_count (function)
macro_rules! Depcrateadd_to_count {
() => {
// Module: crate
// Provides: {"add_to_count"}
// Dependencies: {}
# [doc = " SAFETY: Calling this from more than a single thread at a time is undefined"] # [doc = " behavior, so you *must* guarantee you only call it from a single thread at"] # [doc = " a time."] unsafe fn add_to_count (inc : u32) { unsafe { COUNTER += inc ; } }
};
}
