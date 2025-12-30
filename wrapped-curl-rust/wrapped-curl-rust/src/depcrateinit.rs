// Generated macro for init (function)
macro_rules! Depcrateinit {
() => {
// Module: crate
// Provides: {"init"}
// Dependencies: {}
# [doc = " Initializes the underlying libcurl library."] # [doc = ""] # [doc = " The underlying libcurl library must be initialized before use, and must be"] # [doc = " done so on the main thread before any other threads are created by the"] # [doc = " program. This crate will do this for you automatically in the following"] # [doc = " scenarios:"] # [doc = ""] # [doc = " - Creating a new [`Easy`][easy::Easy] or [`Multi`][multi::Multi] handle"] # [doc = " - At program startup on Windows, macOS, Linux, Android, or FreeBSD systems"] # [doc = ""] # [doc = " This should be sufficient for most applications and scenarios, but in any"] # [doc = " other case, it is strongly recommended that you call this function manually"] # [doc = " as soon as your program starts."] # [doc = ""] # [doc = " Calling this function more than once is harmless and has no effect."] # [inline] pub fn init () { # [doc = " Used to prevent concurrent or duplicate initialization."] static INIT : Once = Once :: new () ; INIT . call_once (| | { # [cfg (need_openssl_init)] openssl_probe :: init_ssl_cert_env_vars () ; # [cfg (need_openssl_init)] openssl_sys :: init () ; unsafe { assert_eq ! (curl_sys :: curl_global_init (curl_sys :: CURL_GLOBAL_ALL) , 0) ; } # [cfg (test)] { INITIALIZED . store (true , std :: sync :: atomic :: Ordering :: SeqCst) ; } }) ; }
};
}
