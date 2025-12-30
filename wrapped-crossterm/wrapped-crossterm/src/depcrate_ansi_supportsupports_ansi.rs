// Generated macro for supports_ansi (function)
macro_rules! Depcrate_ansi_supportsupports_ansi {
() => {
// Module: crate::ansi_support
// Provides: {"supports_ansi"}
// Dependencies: {}
# [doc = " Checks if the current terminal supports ANSI escape sequences"] pub fn supports_ansi () -> bool { INITIALIZER . call_once (| | { let supported = enable_vt_processing () . is_ok () || std :: env :: var ("TERM") . map_or (false , | term | term != "dumb") ; SUPPORTS_ANSI_ESCAPE_CODES . store (supported , Ordering :: SeqCst) ; }) ; SUPPORTS_ANSI_ESCAPE_CODES . load (Ordering :: SeqCst) }
};
}
