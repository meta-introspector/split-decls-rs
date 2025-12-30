// Generated macro for rustls_handshake_kind_str (function)
macro_rules! Depcrate_enumsrustls_handshake_kind_str {
() => {
// Module: crate::enums
// Provides: {"rustls_handshake_kind_str"}
// Dependencies: {}
# [doc = " Convert a `rustls_handshake_kind` to a string with a friendly description of the kind"] # [doc = " of handshake."] # [doc = ""] # [doc = " The returned `rustls_str` has a static lifetime equal to that of the program and does"] # [doc = " not need to be manually freed."] # [no_mangle] pub extern "C" fn rustls_handshake_kind_str (kind : rustls_handshake_kind) -> rustls_str < 'static > { ffi_panic_boundary ! { rustls_str :: from_str_unchecked (match kind { rustls_handshake_kind :: Unknown => "unknown" , rustls_handshake_kind :: Full => "full" , rustls_handshake_kind :: FullWithHelloRetryRequest => "full with hello retry request" , rustls_handshake_kind :: Resumed => "resumed" , }) } }
};
}
