// Generated macro for impl_2872 (impl)
macro_rules! Depcrate_writeimpl_2872 {
() => {
// Module: crate::write
// Provides: {"impl_2872"}
// Dependencies: {}
impl < 'a > fmt :: Display for DisplayValues < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for (i , val) in self . 0 . iter () . enumerate () { if i == 0 { write ! (f , "{val}") ? ; } else { write ! (f , ", {val}") ? ; } } Ok (()) } }
};
}
