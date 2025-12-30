// Generated macro for abort (function)
macro_rules! Depcrateabort {
() => {
// Module: crate
// Provides: {"abort"}
// Dependencies: {}
# [cfg (not (feature = "std"))] fn abort () -> ! { struct PanicOnDrop ; impl Drop for PanicOnDrop { fn drop (& mut self) { panic ! ("Panic while panicking to abort") ; } } let _bomb = PanicOnDrop ; panic ! ("Panic while panicking to abort") }
};
}
