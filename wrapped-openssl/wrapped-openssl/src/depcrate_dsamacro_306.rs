// Generated macro for macro_306 (macro)
macro_rules! Depcrate_dsamacro_306 {
() => {
// Module: crate::dsa
// Provides: {"macro_306"}
// Dependencies: {}
generic_foreign_type_and_impl_send_sync ! { type CType = ffi :: DSA ; fn drop = ffi :: DSA_free ; # [doc = " Object representing DSA keys."] # [doc = ""] # [doc = " A DSA object contains the parameters p, q, and g.  There is a private"] # [doc = " and public key.  The values p, g, and q are:"] # [doc = ""] # [doc = " * `p`: DSA prime parameter"] # [doc = " * `q`: DSA sub-prime parameter"] # [doc = " * `g`: DSA base parameter"] # [doc = ""] # [doc = " These values are used to calculate a pair of asymmetrical keys used for"] # [doc = " signing."] # [doc = ""] # [doc = " OpenSSL documentation at [`DSA_new`]"] # [doc = ""] # [doc = " [`DSA_new`]: https://docs.openssl.org/master/man3/DSA_new/"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::dsa::Dsa;"] # [doc = " use openssl::error::ErrorStack;"] # [doc = " use openssl::pkey::Private;"] # [doc = ""] # [doc = " fn create_dsa() -> Result<Dsa<Private>, ErrorStack> {"] # [doc = "     let sign = Dsa::generate(2048)?;"] # [doc = "     Ok(sign)"] # [doc = " }"] # [doc = " # fn main() {"] # [doc = " #    create_dsa();"] # [doc = " # }"] # [doc = " ```"] pub struct Dsa < T >; # [doc = " Reference to [`Dsa`]."] # [doc = ""] # [doc = " [`Dsa`]: struct.Dsa.html"] pub struct DsaRef < T >; }
};
}
