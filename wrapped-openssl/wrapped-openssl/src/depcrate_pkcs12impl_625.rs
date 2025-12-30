// Generated macro for impl_625 (impl)
macro_rules! Depcrate_pkcs12impl_625 {
() => {
// Module: crate::pkcs12
// Provides: {"impl_625"}
// Dependencies: {}
impl Pkcs12Ref { to_der ! { # [doc = " Serializes the `Pkcs12` to its standard DER encoding."] # [corresponds (i2d_PKCS12)] to_der , ffi :: i2d_PKCS12 } # [doc = " Deprecated."] # [deprecated (note = "Use parse2 instead" , since = "0.10.46")] # [allow (deprecated)] pub fn parse (& self , pass : & str) -> Result < ParsedPkcs12 , ErrorStack > { let parsed = self . parse2 (pass) ? ; Ok (ParsedPkcs12 { pkey : parsed . pkey . unwrap () , cert : parsed . cert . unwrap () , chain : parsed . ca , }) } # [doc = " Extracts the contents of the `Pkcs12`."] # [corresponds (PKCS12_parse)] pub fn parse2 (& self , pass : & str) -> Result < ParsedPkcs12_2 , ErrorStack > { unsafe { let pass = CString :: new (pass . as_bytes ()) . unwrap () ; let mut pkey = ptr :: null_mut () ; let mut cert = ptr :: null_mut () ; let mut ca = ptr :: null_mut () ; cvt (ffi :: PKCS12_parse (self . as_ptr () , pass . as_ptr () , & mut pkey , & mut cert , & mut ca ,)) ? ; let pkey = PKey :: from_ptr_opt (pkey) ; let cert = X509 :: from_ptr_opt (cert) ; let ca = Stack :: from_ptr_opt (ca) ; Ok (ParsedPkcs12_2 { pkey , cert , ca }) } } }
};
}
