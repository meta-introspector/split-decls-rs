// Generated macro for impl_48 (impl)
macro_rules! Depcrate_snakeimpl_48 {
() => {
// Module: crate::snake
// Provides: {"impl_48"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsSnakeCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , lowercase , | f | write ! (f , "_") , f) } }
};
}
