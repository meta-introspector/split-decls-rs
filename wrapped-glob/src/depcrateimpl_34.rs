// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl fmt :: Display for PatternError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Pattern syntax error near position {}: {}" , self . pos , self . msg) } }
};
}
