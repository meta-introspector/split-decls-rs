// Generated macro for init (function)
macro_rules! Depcrateinit {
() => {
// Module: crate
// Provides: {"init"}
// Dependencies: {}
# [inline] # [doc = " Initialize the *AWS-LC* library. (This should generally not be needed.)"] pub fn init () { START . call_once (| | unsafe { CRYPTO_library_init () ; }) ; }
};
}
