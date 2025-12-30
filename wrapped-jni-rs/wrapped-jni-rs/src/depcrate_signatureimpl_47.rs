// Generated macro for impl_47 (impl)
macro_rules! Depcrate_signatureimpl_47 {
() => {
// Module: crate::signature
// Provides: {"impl_47"}
// Dependencies: {}
impl fmt :: Display for TypeSignature { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "(") ? ; for a in & self . args { write ! (f , "{a}") ? ; } write ! (f , ")") ? ; write ! (f , "{}" , self . ret) ? ; Ok (()) } }
};
}
