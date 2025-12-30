// Generated macro for impl_121 (impl)
macro_rules! Depcrate_utilsimpl_121 {
() => {
// Module: crate::utils
// Provides: {"impl_121"}
// Dependencies: {}
impl fmt :: Display for Attributes { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for ansi in self . bits () . map (| bit | bit + 1) { write ! (f , "\x1b[{ansi}m") ? ; } Ok (()) } }
};
}
