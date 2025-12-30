// Generated macro for impl_76 (impl)
macro_rules! Depcrate_errorimpl_76 {
() => {
// Module: crate::error
// Provides: {"impl_76"}
// Dependencies: {}
impl fmt :: Display for CargoError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "Cargo command failed: {}" , self . kind) ? ; if let Some (ref context) = self . context { writeln ! (f , "{context}") ? ; } if let Some (ref cause) = self . cause { writeln ! (f , "Cause: {cause}") ? ; } Ok (()) } }
};
}
