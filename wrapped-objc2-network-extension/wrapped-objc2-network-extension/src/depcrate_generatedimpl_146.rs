// Generated macro for impl_146 (impl)
macro_rules! Depcrate_generatedimpl_146 {
() => {
// Module: crate::generated
// Provides: {"impl_146"}
// Dependencies: {}
impl NEDNSOverTLSSettings { extern_methods ! (# [doc = " The name of the server to use for TLS certificate validation."] # [unsafe (method (serverName))] # [unsafe (method_family = none)] pub unsafe fn serverName (& self) -> Option < Retained < NSString >>; # [doc = " Setter for [`serverName`][Self::serverName]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setServerName :))] # [unsafe (method_family = none)] pub unsafe fn setServerName (& self , server_name : Option <& NSString >) ; # [doc = " The optional certificate identity keychain reference to use as a TLS client certificate."] # [unsafe (method (identityReference))] # [unsafe (method_family = none)] pub unsafe fn identityReference (& self) -> Option < Retained < NSData >>; # [doc = " Setter for [`identityReference`][Self::identityReference]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setIdentityReference :))] # [unsafe (method_family = none)] pub unsafe fn setIdentityReference (& self , identity_reference : Option <& NSData >) ;) ; }
};
}
