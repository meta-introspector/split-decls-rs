// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let desc = self . description () ; match self . extra { Some (ref s) => write ! (f , "[{}] {} ({})" , self . code () , desc , s) , None => write ! (f , "[{}] {}" , self . code () , desc) , } } }
};
}
