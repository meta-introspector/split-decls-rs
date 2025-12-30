// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_remoteimpl_1010 {
() => {
// Module: crate::remote
// Provides: {"impl_1010"}
// Dependencies: {}
impl < 'cb > Binding for PushOptions < 'cb > { type Raw = raw :: git_push_options ; unsafe fn from_raw (_raw : raw :: git_push_options) -> PushOptions < 'cb > { panic ! ("unimplemented") ; } fn raw (& self) -> raw :: git_push_options { raw :: git_push_options { version : 1 , callbacks : self . callbacks . as_ref () . map (| m | m . raw ()) . unwrap_or_else (| | RemoteCallbacks :: new () . raw ()) , proxy_opts : self . proxy . as_ref () . map (| m | m . raw ()) . unwrap_or_else (| | ProxyOptions :: new () . raw ()) , pb_parallelism : self . pb_parallelism as libc :: c_uint , follow_redirects : self . follow_redirects . raw () , custom_headers : git_strarray { count : self . custom_headers_ptrs . len () , strings : self . custom_headers_ptrs . as_ptr () as * mut _ , } , remote_push_options : git_strarray { count : self . remote_push_options . len () , strings : self . remote_push_options_ptrs . as_ptr () as * mut _ , } , } } }
};
}
