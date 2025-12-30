// Generated macro for impl_885 (impl)
macro_rules! Depcrate_proxy_optionsimpl_885 {
() => {
// Module: crate::proxy_options
// Provides: {"impl_885"}
// Dependencies: {}
impl < 'a > ProxyOptions < 'a > { # [doc = " Creates a new set of proxy options ready to be configured."] pub fn new () -> ProxyOptions < 'a > { Default :: default () } # [doc = " Try to auto-detect the proxy from the git configuration."] # [doc = ""] # [doc = " Note that this will override `url` specified before."] pub fn auto (& mut self) -> & mut Self { self . proxy_kind = raw :: GIT_PROXY_AUTO ; self } # [doc = " Specify the exact URL of the proxy to use."] # [doc = ""] # [doc = " Note that this will override `auto` specified before."] pub fn url (& mut self , url : & str) -> & mut Self { self . proxy_kind = raw :: GIT_PROXY_SPECIFIED ; self . url = Some (CString :: new (url) . unwrap ()) ; self } }
};
}
