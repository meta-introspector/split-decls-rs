// Generated macro for impl_119 (impl)
macro_rules! Depcrate_agreementimpl_119 {
() => {
// Module: crate::agreement
// Provides: {"impl_119"}
// Dependencies: {}
impl < B : AsRef < [u8] > > UnparsedPublicKey < B > { # [doc = " Constructs a new `UnparsedPublicKey`."] pub fn new (algorithm : & 'static Algorithm , bytes : B) -> Self { UnparsedPublicKey { alg : algorithm , bytes , } } # [doc = " The agreement algorithm associated with this public key"] pub fn algorithm (& self) -> & 'static Algorithm { self . alg } # [doc = " The bytes provided for this public key"] pub fn bytes (& self) -> & B { & self . bytes } }
};
}
