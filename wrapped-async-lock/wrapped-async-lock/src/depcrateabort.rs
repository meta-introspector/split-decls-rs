// Generated macro for abort (function)
macro_rules! Depcrateabort {
() => {
// Module: crate
// Provides: {"abort"}
// Dependencies: {}
# [cold] fn abort () -> ! { # [cfg (not (feature = "std"))] { struct Bomb ; impl Drop for Bomb { fn drop (& mut self) { panic ! ("Panicking while panicking to abort") } } let _bomb = Bomb ; panic ! ("Panicking while panicking to abort") } # [cfg (feature = "std")] std :: process :: abort () }
};
}
