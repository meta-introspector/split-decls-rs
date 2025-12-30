// Generated macro for impl_83 (impl)
macro_rules! Depcrate_tagimpl_83 {
() => {
// Module: crate::tag
// Provides: {"impl_83"}
// Dependencies: {}
impl Display for TagMode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { TagMode :: Explicit => f . write_str ("EXPLICIT") , TagMode :: Implicit => f . write_str ("IMPLICIT") , } } }
};
}
