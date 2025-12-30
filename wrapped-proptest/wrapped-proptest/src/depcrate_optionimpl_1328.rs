// Generated macro for impl_1328 (impl)
macro_rules! Depcrate_optionimpl_1328 {
() => {
// Module: crate::option
// Provides: {"impl_1328"}
// Dependencies: {}
impl < T : Strategy + fmt :: Debug > fmt :: Debug for OptionStrategy < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "OptionStrategy({:?})" , self . 0) } }
};
}
