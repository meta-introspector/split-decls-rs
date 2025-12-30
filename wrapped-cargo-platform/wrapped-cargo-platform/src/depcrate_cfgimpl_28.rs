// Generated macro for impl_28 (impl)
macro_rules! Depcrate_cfgimpl_28 {
() => {
// Module: crate::cfg
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , T : fmt :: Display > fmt :: Display for CommaSep < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , v) in self . 0 . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{}" , v) ? ; } Ok (()) } }
};
}
