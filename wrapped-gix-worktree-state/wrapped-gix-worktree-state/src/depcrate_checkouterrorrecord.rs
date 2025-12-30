// Generated macro for ErrorRecord (struct)
macro_rules! Depcrate_checkoutErrorRecord {
() => {
// Module: crate::checkout
// Provides: {"ErrorRecord"}
// Dependencies: {}
# [doc = " A path that encountered an IO error."] # [derive (Debug)] pub struct ErrorRecord { # [doc = " the path that encountered the error."] pub path : BString , # [doc = " The error"] pub error : Box < dyn std :: error :: Error + Send + Sync + 'static > , }
};
}
