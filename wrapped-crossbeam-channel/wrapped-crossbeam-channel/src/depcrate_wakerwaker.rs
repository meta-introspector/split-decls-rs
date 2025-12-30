// Generated macro for Waker (struct)
macro_rules! Depcrate_wakerWaker {
() => {
// Module: crate::waker
// Provides: {"Waker"}
// Dependencies: {}
# [doc = " A queue of threads blocked on channel operations."] # [doc = ""] # [doc = " This data structure is used by threads to register blocking operations and get woken up once"] # [doc = " an operation becomes ready."] pub (crate) struct Waker { # [doc = " A list of select operations."] selectors : Vec < Entry > , # [doc = " A list of operations waiting to be ready."] observers : Vec < Entry > , }
};
}
