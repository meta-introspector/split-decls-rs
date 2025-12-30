// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_remoteimpl_1007 {
() => {
// Module: crate::remote
// Provides: {"impl_1007"}
// Dependencies: {}
impl < 'cb > Binding for FetchOptions < 'cb > { type Raw = raw :: git_fetch_options ; unsafe fn from_raw (_raw : raw :: git_fetch_options) -> FetchOptions < 'cb > { panic ! ("unimplemented") ; } fn raw (& self) -> raw :: git_fetch_options { raw :: git_fetch_options { version : 1 , callbacks : self . callbacks . as_ref () . map (| m | m . raw ()) . unwrap_or_else (| | RemoteCallbacks :: new () . raw ()) , proxy_opts : self . proxy . as_ref () . map (| m | m . raw ()) . unwrap_or_else (| | ProxyOptions :: new () . raw ()) , prune : crate :: call :: convert (& self . prune) , update_fetchhead : self . update_flags . bits () as c_uint , download_tags : crate :: call :: convert (& self . download_tags) , depth : self . depth , follow_redirects : self . follow_redirects . raw () , custom_headers : git_strarray { count : self . custom_headers_ptrs . len () , strings : self . custom_headers_ptrs . as_ptr () as * mut _ , } , } } }
};
}
