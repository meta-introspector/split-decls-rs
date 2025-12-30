// Generated macro for impl_1414 (impl)
macro_rules! Depcrate_x509_extensionimpl_1414 {
() => {
// Module: crate::x509::extension
// Provides: {"impl_1414"}
// Dependencies: {}
impl AuthorityKeyIdentifier { # [doc = " Construct a new `AuthorityKeyIdentifier` extension."] pub fn new () -> AuthorityKeyIdentifier { AuthorityKeyIdentifier { critical : false , keyid : None , issuer : None , } } # [doc = " Sets the `critical` flag to `true`. The extension will be critical."] pub fn critical (& mut self) -> & mut AuthorityKeyIdentifier { self . critical = true ; self } # [doc = " Sets the `keyid` flag."] pub fn keyid (& mut self , always : bool) -> & mut AuthorityKeyIdentifier { self . keyid = Some (always) ; self } # [doc = " Sets the `issuer` flag."] pub fn issuer (& mut self , always : bool) -> & mut AuthorityKeyIdentifier { self . issuer = Some (always) ; self } # [doc = " Return a `AuthorityKeyIdentifier` extension as an `X509Extension`."] # [allow (deprecated)] pub fn build (& self , ctx : & X509v3Context < '_ >) -> Result < X509Extension , ErrorStack > { let mut value = String :: new () ; let mut first = true ; append (& mut value , & mut first , self . critical , "critical") ; match self . keyid { Some (true) => append (& mut value , & mut first , true , "keyid:always") , Some (false) => append (& mut value , & mut first , true , "keyid") , None => { } } match self . issuer { Some (true) => append (& mut value , & mut first , true , "issuer:always") , Some (false) => append (& mut value , & mut first , true , "issuer") , None => { } } X509Extension :: new_nid (None , Some (ctx) , Nid :: AUTHORITY_KEY_IDENTIFIER , & value) } }
};
}
