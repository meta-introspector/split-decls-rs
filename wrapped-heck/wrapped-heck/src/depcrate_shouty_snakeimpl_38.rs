// Generated macro for impl_38 (impl)
macro_rules! Depcrate_shouty_snakeimpl_38 {
() => {
// Module: crate::shouty_snake
// Provides: {"impl_38"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsShoutySnakeCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , uppercase , | f | write ! (f , "_") , f) } }
};
}
