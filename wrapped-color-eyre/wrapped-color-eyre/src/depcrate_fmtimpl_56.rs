// Generated macro for impl_56 (impl)
macro_rules! Depcrate_fmtimpl_56 {
() => {
// Module: crate::fmt
// Provides: {"impl_56"}
// Dependencies: {}
impl fmt :: Display for LocationSection < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let theme = self . 1 ; if let Some (loc) = self . 0 { write ! (f , "{}" , loc . file () . style (theme . panic_file)) ? ; write ! (f , ":") ? ; write ! (f , "{}" , loc . line () . style (theme . panic_line_number)) ? ; } else { write ! (f , "<unknown>") ? ; } Ok (()) } }
};
}
