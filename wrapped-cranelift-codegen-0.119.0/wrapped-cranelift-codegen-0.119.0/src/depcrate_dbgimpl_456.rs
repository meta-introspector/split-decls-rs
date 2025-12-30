// Generated macro for impl_456 (impl)
macro_rules! Depcrate_dbgimpl_456 {
() => {
// Module: crate::dbg
// Provides: {"impl_456"}
// Dependencies: {}
impl < 'a , T > fmt :: Display for DisplayList < 'a , T > where T : 'a + fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 . split_first () { None => write ! (f , "[]") , Some ((first , rest)) => { write ! (f , "[{first}") ? ; for x in rest { write ! (f , ", {x}") ? ; } write ! (f , "]") } } } }
};
}
