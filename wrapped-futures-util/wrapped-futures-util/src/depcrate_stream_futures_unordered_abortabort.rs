// Generated macro for abort (function)
macro_rules! Depcrate_stream_futures_unordered_abortabort {
() => {
// Module: crate::stream::futures_unordered::abort
// Provides: {"abort"}
// Dependencies: {}
pub (super) fn abort (s : & str) -> ! { struct DoublePanic ; impl Drop for DoublePanic { fn drop (& mut self) { panic ! ("panicking twice to abort the program") ; } } let _bomb = DoublePanic ; panic ! ("{}" , s) ; }
};
}
