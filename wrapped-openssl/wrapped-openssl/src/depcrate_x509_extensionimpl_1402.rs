// Generated macro for impl_1402 (impl)
macro_rules! Depcrate_x509_extensionimpl_1402 {
() => {
// Module: crate::x509::extension
// Provides: {"impl_1402"}
// Dependencies: {}
impl BasicConstraints { # [doc = " Construct a new `BasicConstraints` extension."] pub fn new () -> BasicConstraints { BasicConstraints { critical : false , ca : false , pathlen : None , } } # [doc = " Sets the `critical` flag to `true`. The extension will be critical."] pub fn critical (& mut self) -> & mut BasicConstraints { self . critical = true ; self } # [doc = " Sets the `ca` flag to `true`."] pub fn ca (& mut self) -> & mut BasicConstraints { self . ca = true ; self } # [doc = " Sets the `pathlen` to an optional non-negative value. The `pathlen` is the"] # [doc = " maximum number of CAs that can appear below this one in a chain."] pub fn pathlen (& mut self , pathlen : u32) -> & mut BasicConstraints { self . pathlen = Some (pathlen) ; self } # [doc = " Return the `BasicConstraints` extension as an `X509Extension`."] # [allow (deprecated)] pub fn build (& self) -> Result < X509Extension , ErrorStack > { let mut value = String :: new () ; if self . critical { value . push_str ("critical,") ; } value . push_str ("CA:") ; if self . ca { value . push_str ("TRUE") ; } else { value . push_str ("FALSE") ; } if let Some (pathlen) = self . pathlen { write ! (value , ",pathlen:{}" , pathlen) . unwrap () ; } X509Extension :: new_nid (None , None , Nid :: BASIC_CONSTRAINTS , & value) } }
};
}
