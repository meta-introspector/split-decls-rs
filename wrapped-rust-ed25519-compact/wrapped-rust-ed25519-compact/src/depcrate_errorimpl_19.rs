// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: SignatureMismatch => write ! (f , "Signature doesn't verify") , Error :: WeakPublicKey => write ! (f , "Weak public key") , Error :: InvalidPublicKey => write ! (f , "Invalid public key") , Error :: InvalidSecretKey => write ! (f , "Invalid secret key") , Error :: InvalidSignature => write ! (f , "Invalid signature") , Error :: InvalidSeed => write ! (f , "Invalid seed length") , Error :: InvalidBlind => write ! (f , "Invalid blind length") , Error :: InvalidNoise => write ! (f , "Invalid noise length") , Error :: ParseError => write ! (f , "Parse error") , Error :: NonCanonical => write ! (f , "Non-canonical encoding") , } } }
};
}
