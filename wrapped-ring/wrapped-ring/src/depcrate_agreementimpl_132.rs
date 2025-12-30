// Generated macro for impl_132 (impl)
macro_rules! Depcrate_agreementimpl_132 {
() => {
// Module: crate::agreement
// Provides: {"impl_132"}
// Dependencies: {}
impl < B > UnparsedPublicKey < B > { # [doc = " Constructs a new `UnparsedPublicKey`."] pub fn new (algorithm : & 'static Algorithm , bytes : B) -> Self { Self { algorithm , bytes } } # [doc = " The algorithm for the public key."] # [inline] pub fn algorithm (& self) -> & 'static Algorithm { self . algorithm } # [doc = " TODO: doc"] # [inline] pub fn bytes (& self) -> & B { & self . bytes } }
};
}
