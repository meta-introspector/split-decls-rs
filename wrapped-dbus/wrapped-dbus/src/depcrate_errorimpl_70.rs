// Generated macro for impl_70 (impl)
macro_rules! Depcrate_errorimpl_70 {
() => {
// Module: crate::error
// Provides: {"impl_70"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { if let Some (x) = self . message () { write ! (f , "{}" , x) } else { Ok (()) } } }
};
}
