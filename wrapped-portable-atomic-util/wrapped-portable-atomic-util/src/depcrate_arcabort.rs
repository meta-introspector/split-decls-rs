// Generated macro for abort (function)
macro_rules! Depcrate_arcabort {
() => {
// Module: crate::arc
// Provides: {"abort"}
// Dependencies: {}
# [cfg (not (feature = "std"))] # [cold] fn abort () -> ! { struct Abort ; impl Drop for Abort { fn drop (& mut self) { panic ! () ; } } let _abort = Abort ; panic ! ("abort") }
};
}
