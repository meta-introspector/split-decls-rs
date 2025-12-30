// Generated macro for impl_412 (impl)
macro_rules! Depcrate_options_parserimpl_412 {
() => {
// Module: crate::options::parser
// Provides: {"impl_412"}
// Dependencies: {}
impl fmt :: Display for Arg { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { write ! (f , "--{}" , self . long) ? ; if let Some (short) = self . short { write ! (f , " (-{})" , short as char) ? ; } Ok (()) } }
};
}
