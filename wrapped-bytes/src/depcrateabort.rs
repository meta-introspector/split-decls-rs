// Generated macro for abort (function)
macro_rules! Depcrateabort {
() => {
// Module: crate
// Provides: {"abort"}
// Dependencies: {}
# [inline (never)] # [cold] fn abort () -> ! { # [cfg (feature = "std")] { std :: process :: abort () ; } # [cfg (not (feature = "std"))] { struct Abort ; impl Drop for Abort { fn drop (& mut self) { panic ! () ; } } let _a = Abort ; panic ! ("abort") ; } }
};
}
