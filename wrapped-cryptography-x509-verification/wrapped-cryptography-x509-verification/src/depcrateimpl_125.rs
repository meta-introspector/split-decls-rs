// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl < B : CryptoOps > Display for ValidationError < '_ , B > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . kind { ValidationErrorKind :: CandidatesExhausted (inner) => { write ! (f , "candidates exhausted: {inner}") } ValidationErrorKind :: Malformed (err) => err . fmt (f) , ValidationErrorKind :: ExtensionError { oid , reason } => { write ! (f , "invalid extension: {oid}: {reason}") } ValidationErrorKind :: FatalError (err) => write ! (f , "fatal error: {err}") , ValidationErrorKind :: Other (err) => write ! (f , "{err}") , } } }
};
}
