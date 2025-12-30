// Generated macro for impl_2829 (impl)
macro_rules! Depcrate_verifierimpl_2829 {
() => {
// Module: crate::verifier
// Provides: {"impl_2829"}
// Dependencies: {}
impl Display for VerifierError { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match & self . context { None => write ! (f , "{}: {}" , self . location , self . message) , Some (context) => write ! (f , "{} ({}): {}" , self . location , context , self . message) , } } }
};
}
