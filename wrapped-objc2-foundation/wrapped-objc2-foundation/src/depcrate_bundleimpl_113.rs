// Generated macro for impl_113 (impl)
macro_rules! Depcrate_bundleimpl_113 {
() => {
// Module: crate::bundle
// Provides: {"impl_113"}
// Dependencies: {}
impl NSBundle { # [cfg (feature = "NSString")] # [cfg (feature = "NSDictionary")] pub fn name (& self) -> Option < objc2 :: rc :: Retained < crate :: NSString > > { let info = self . infoDictionary () ? ; let name = info . objectForKey (crate :: ns_string ! ("CFBundleName")) ? ; Some (name . downcast () . expect ("CFBundleName to be NSString")) } }
};
}
