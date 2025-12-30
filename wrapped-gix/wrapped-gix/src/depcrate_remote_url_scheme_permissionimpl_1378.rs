// Generated macro for impl_1378 (impl)
macro_rules! Depcrate_remote_url_scheme_permissionimpl_1378 {
() => {
// Module: crate::remote::url::scheme_permission
// Provides: {"impl_1378"}
// Dependencies: {}
impl Allow { # [doc = " Return true if we represent something like 'allow == true'."] pub fn to_bool (self , user_allowed : Option < bool >) -> bool { match self { Allow :: Always => true , Allow :: Never => false , Allow :: User => user_allowed . unwrap_or (true) , } } }
};
}
