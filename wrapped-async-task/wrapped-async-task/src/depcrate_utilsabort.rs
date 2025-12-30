// Generated macro for abort (function)
macro_rules! Depcrate_utilsabort {
() => {
// Module: crate::utils
// Provides: {"abort"}
// Dependencies: {}
# [doc = " Aborts the process."] # [doc = ""] # [doc = " To abort, this function simply panics while panicking."] pub (crate) fn abort () -> ! { struct Panic ; impl Drop for Panic { fn drop (& mut self) { panic ! ("aborting the process") ; } } let _panic = Panic ; panic ! ("aborting the process") ; }
};
}
