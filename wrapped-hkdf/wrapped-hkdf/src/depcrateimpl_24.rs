// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < H : HmacImpl > GenericHkdfExtract < H > { # [doc = " Initiates the HKDF-Extract context with the given optional salt"] pub fn new (salt : Option < & [u8] >) -> Self { let default_salt = Output :: < H > :: default () ; let salt = salt . unwrap_or (& default_salt) ; let hmac = H :: new_from_slice (salt) ; Self { hmac } } # [doc = " Feeds in additional input key material to the HKDF-Extract context"] pub fn input_ikm (& mut self , ikm : & [u8]) { self . hmac . update (ikm) ; } # [doc = " Completes the HKDF-Extract operation, returning both the generated pseudorandom key and"] # [doc = " `Hkdf` struct for expanding."] pub fn finalize (self) -> (Output < H > , GenericHkdf < H >) { let prk = self . hmac . finalize () ; let hkdf = GenericHkdf :: < H > :: from_prk (& prk) . expect ("PRK size is correct") ; (prk , hkdf) } }
};
}
