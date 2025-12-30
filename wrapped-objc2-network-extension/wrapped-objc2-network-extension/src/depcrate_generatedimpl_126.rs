// Generated macro for impl_126 (impl)
macro_rules! Depcrate_generatedimpl_126 {
() => {
// Module: crate::generated
// Provides: {"impl_126"}
// Dependencies: {}
impl NEDNSProxyProviderProtocol { extern_methods ! (# [doc = " A dictionary containing NEDNSProxyProvider vendor-specific configuration parameters. This dictionary is passed as-is to NEDNSProxyProviders when a DNS proxy is started."] # [unsafe (method (providerConfiguration))] # [unsafe (method_family = none)] pub unsafe fn providerConfiguration (& self ,) -> Option < Retained < NSDictionary < NSString , AnyObject >>>; # [doc = " Setter for [`providerConfiguration`][Self::providerConfiguration]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `provider_configuration` generic should be of the correct type."] # [unsafe (method (setProviderConfiguration :))] # [unsafe (method_family = none)] pub unsafe fn setProviderConfiguration (& self , provider_configuration : Option <& NSDictionary < NSString , AnyObject >>,) ; # [doc = " A string containing the bundle identifier of the NEDNSProxyProvider to be used by this configuration."] # [unsafe (method (providerBundleIdentifier))] # [unsafe (method_family = none)] pub unsafe fn providerBundleIdentifier (& self) -> Option < Retained < NSString >>; # [doc = " Setter for [`providerBundleIdentifier`][Self::providerBundleIdentifier]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setProviderBundleIdentifier :))] # [unsafe (method_family = none)] pub unsafe fn setProviderBundleIdentifier (& self , provider_bundle_identifier : Option <& NSString >,) ;) ; }
};
}
