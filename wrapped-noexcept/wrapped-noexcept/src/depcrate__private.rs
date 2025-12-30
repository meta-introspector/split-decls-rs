// Generated macro for __private (module)
macro_rules! Depcrate__private {
() => {
// Module: crate
// Provides: {"__private"}
// Dependencies: {}
# [doc (hidden)] pub mod __private { # [doc (hidden)] pub struct AbortOnDrop ; impl Drop for AbortOnDrop { # [inline] fn drop (& mut self) { abort () ; } } # [inline] fn abort () -> ! { panic ! ("panic inside of #[abort_on_panic]") ; } }
};
}
