// Generated macro for impl_192 (impl)
macro_rules! Depcrate_internalimpl_192 {
() => {
// Module: crate::internal
// Provides: {"impl_192"}
// Dependencies: {}
impl < T > Err < (T , ErrorKind) > { # [doc = " Maps `Err<(T, ErrorKind)>` to `Err<(U, ErrorKind)>` with the given `F: T -> U`"] pub fn map_input < U , F > (self , f : F) -> Err < (U , ErrorKind) > where F : FnOnce (T) -> U , { match self { Err :: Incomplete (n) => Err :: Incomplete (n) , Err :: Failure ((input , k)) => Err :: Failure ((f (input) , k)) , Err :: Error ((input , k)) => Err :: Error ((f (input) , k)) , } } }
};
}
