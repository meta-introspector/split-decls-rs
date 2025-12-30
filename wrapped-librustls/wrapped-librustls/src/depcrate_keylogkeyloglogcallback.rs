// Generated macro for KeylogLogCallback (type)
macro_rules! Depcrate_keylogKeylogLogCallback {
() => {
// Module: crate::keylog
// Provides: {"KeylogLogCallback"}
// Dependencies: {}
# [doc = " A type alias for a keylog log callback that has been extracted from an option."] pub (crate) type KeylogLogCallback = unsafe extern "C" fn (label : rustls_str , client_random : * const u8 , client_random_len : usize , secret : * const u8 , secret_len : usize ,) ;
};
}
