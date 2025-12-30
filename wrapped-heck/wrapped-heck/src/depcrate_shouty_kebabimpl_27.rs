// Generated macro for impl_27 (impl)
macro_rules! Depcrate_shouty_kebabimpl_27 {
() => {
// Module: crate::shouty_kebab
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsShoutyKebabCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , uppercase , | f | write ! (f , "-") , f) } }
};
}
