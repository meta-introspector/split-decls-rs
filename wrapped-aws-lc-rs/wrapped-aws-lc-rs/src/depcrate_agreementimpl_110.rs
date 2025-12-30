// Generated macro for impl_110 (impl)
macro_rules! Depcrate_agreementimpl_110 {
() => {
// Module: crate::agreement
// Provides: {"impl_110"}
// Dependencies: {}
impl Debug for PublicKey { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . write_str (& format ! ("PublicKey {{ algorithm: {:?}, bytes: \"{}\" }}" , self . inner_key . algorithm () , hex :: encode (& self . key_bytes [0 .. self . len]))) } }
};
}
