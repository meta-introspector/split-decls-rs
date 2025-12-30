// Generated macro for impl_1382 (impl)
macro_rules! Depcrate_remote_url_scheme_permissionimpl_1382 {
() => {
// Module: crate::remote::url::scheme_permission
// Provides: {"impl_1382"}
// Dependencies: {}
# [doc = " Access"] impl SchemePermission { pub fn allow (& self , scheme : & gix_url :: Scheme) -> bool { self . allow_per_scheme . get (scheme) . or (self . allow . as_ref ()) . map_or_else (| | { use gix_url :: Scheme :: * ; match scheme { File | Git | Ssh | Http | Https => true , Ext (_) => false , } } , | allow | allow . to_bool (self . user_allowed) ,) } }
};
}
