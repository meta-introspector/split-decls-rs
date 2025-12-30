// Generated macro for impl_155 (impl)
macro_rules! Depcrate_generatedimpl_155 {
() => {
// Module: crate::generated
// Provides: {"impl_155"}
// Dependencies: {}
impl NEDNSOverHTTPSSettings { extern_methods ! (# [doc = " The URL to which to make DNS-over-HTTPS requests. The format should be an HTTPS URL with the path indicating the location of the DNS-over-HTTPS server, such as: \"https://dnsserver.example.net/dns-query\"."] # [unsafe (method (serverURL))] # [unsafe (method_family = none)] pub unsafe fn serverURL (& self) -> Option < Retained < NSURL >>; # [doc = " Setter for [`serverURL`][Self::serverURL]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setServerURL :))] # [unsafe (method_family = none)] pub unsafe fn setServerURL (& self , server_url : Option <& NSURL >) ; # [doc = " The optional certificate identity keychain reference to use as a TLS client certificate."] # [unsafe (method (identityReference))] # [unsafe (method_family = none)] pub unsafe fn identityReference (& self) -> Option < Retained < NSData >>; # [doc = " Setter for [`identityReference`][Self::identityReference]."] # [doc = ""] # [doc = " This is [copied][objc2_foundation::NSCopying::copy] when set."] # [unsafe (method (setIdentityReference :))] # [unsafe (method_family = none)] pub unsafe fn setIdentityReference (& self , identity_reference : Option <& NSData >) ;) ; }
};
}
