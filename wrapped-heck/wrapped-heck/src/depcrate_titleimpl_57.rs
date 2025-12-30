// Generated macro for impl_57 (impl)
macro_rules! Depcrate_titleimpl_57 {
() => {
// Module: crate::title
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : AsRef < str > > fmt :: Display for AsTitleCase < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { transform (self . 0 . as_ref () , capitalize , | f | write ! (f , " ") , f) } }
};
}
