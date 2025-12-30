// Generated macro for FetchOptions (struct)
macro_rules! Depcrate_remoteFetchOptions {
() => {
// Module: crate::remote
// Provides: {"FetchOptions"}
// Dependencies: {}
# [doc = " Options which can be specified to various fetch operations."] pub struct FetchOptions < 'cb > { callbacks : Option < RemoteCallbacks < 'cb > > , depth : i32 , proxy : Option < ProxyOptions < 'cb > > , prune : FetchPrune , update_flags : RemoteUpdateFlags , download_tags : AutotagOption , follow_redirects : RemoteRedirect , custom_headers : Vec < CString > , custom_headers_ptrs : Vec < * const c_char > , }
};
}
