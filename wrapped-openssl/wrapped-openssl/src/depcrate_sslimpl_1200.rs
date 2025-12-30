// Generated macro for impl_1200 (impl)
macro_rules! Depcrate_sslimpl_1200 {
() => {
// Module: crate::ssl
// Provides: {"impl_1200"}
// Dependencies: {}
impl SslContext { # [doc = " Creates a new builder object for an `SslContext`."] pub fn builder (method : SslMethod) -> Result < SslContextBuilder , ErrorStack > { SslContextBuilder :: new (method) } # [doc = " Returns a new extra data index."] # [doc = ""] # [doc = " Each invocation of this function is guaranteed to return a distinct index. These can be used"] # [doc = " to store data in the context that can be retrieved later by callbacks, for example."] # [corresponds (SSL_CTX_get_ex_new_index)] pub fn new_ex_index < T > () -> Result < Index < SslContext , T > , ErrorStack > where T : 'static + Sync + Send , { unsafe { ffi :: init () ; # [cfg (any (boringssl , awslc))] let idx = cvt_n (get_new_idx (Some (free_data_box :: < T >))) ? ; # [cfg (not (any (boringssl , awslc)))] let idx = cvt_n (get_new_idx (free_data_box :: < T >)) ? ; Ok (Index :: from_raw (idx)) } } fn cached_ex_index < T > () -> Index < SslContext , T > where T : 'static + Sync + Send , { unsafe { let idx = * INDEXES . lock () . unwrap_or_else (| e | e . into_inner ()) . entry (TypeId :: of :: < T > ()) . or_insert_with (| | SslContext :: new_ex_index :: < T > () . unwrap () . as_raw ()) ; Index :: from_raw (idx) } } }
};
}
