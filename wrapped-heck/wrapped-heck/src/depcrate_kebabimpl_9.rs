// Generated macro for impl_9 (impl)
macro_rules! Depcrate_kebabimpl_9 {
() => {
// Module: crate::kebab
// Provides: {"impl_9"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsKebabCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , lowercase , | f | write ! (f , "-") , f) } }
};
}
