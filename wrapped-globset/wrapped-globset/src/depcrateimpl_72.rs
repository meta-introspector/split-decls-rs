// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . glob { None => self . kind . fmt (f) , Some (ref glob) => { write ! (f , "error parsing glob '{}': {}" , glob , self . kind) } } } }
};
}
