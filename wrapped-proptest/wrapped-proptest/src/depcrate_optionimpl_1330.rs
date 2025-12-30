// Generated macro for impl_1330 (impl)
macro_rules! Depcrate_optionimpl_1330 {
() => {
// Module: crate::option
// Provides: {"impl_1330"}
// Dependencies: {}
impl < T : Strategy > fmt :: Debug for OptionValueTree < T > where T :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "OptionValueTree({:?})" , self . 0) } }
};
}
