// Generated macro for impl_458 (impl)
macro_rules! Depcrate_signatureimpl_458 {
() => {
// Module: crate::signature
// Provides: {"impl_458"}
// Dependencies: {}
impl < B > UnparsedPublicKey < B > { # [doc = " Construct a new `UnparsedPublicKey`."] # [doc = ""] # [doc = " No validation of `bytes` is done until `verify()` is called."] # [inline] pub fn new (algorithm : & 'static dyn VerificationAlgorithm , bytes : B) -> Self { Self { algorithm , bytes } } # [doc = " Parses the public key and verifies `signature` is a valid signature of"] # [doc = " `message` using it."] # [doc = ""] # [doc = " See the [crate::signature] module-level documentation for examples."] pub fn verify (& self , message : & [u8] , signature : & [u8]) -> Result < () , error :: Unspecified > where B : AsRef < [u8] > , { let _ = cpu :: features () ; self . algorithm . verify_ (untrusted :: Input :: from (self . bytes . as_ref ()) , untrusted :: Input :: from (message) , untrusted :: Input :: from (signature) , sealed :: Arg ,) } }
};
}
