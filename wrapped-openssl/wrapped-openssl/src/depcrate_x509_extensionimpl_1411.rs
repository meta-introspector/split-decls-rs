// Generated macro for impl_1411 (impl)
macro_rules! Depcrate_x509_extensionimpl_1411 {
() => {
// Module: crate::x509::extension
// Provides: {"impl_1411"}
// Dependencies: {}
impl SubjectKeyIdentifier { # [doc = " Construct a new `SubjectKeyIdentifier` extension."] pub fn new () -> SubjectKeyIdentifier { SubjectKeyIdentifier { critical : false } } # [doc = " Sets the `critical` flag to `true`. The extension will be critical."] pub fn critical (& mut self) -> & mut SubjectKeyIdentifier { self . critical = true ; self } # [doc = " Return a `SubjectKeyIdentifier` extension as an `X509Extension`."] # [allow (deprecated)] pub fn build (& self , ctx : & X509v3Context < '_ >) -> Result < X509Extension , ErrorStack > { let mut value = String :: new () ; let mut first = true ; append (& mut value , & mut first , self . critical , "critical") ; append (& mut value , & mut first , true , "hash") ; X509Extension :: new_nid (None , Some (ctx) , Nid :: SUBJECT_KEY_IDENTIFIER , & value) } }
};
}
