// Generated macro for impl_275 (impl)
macro_rules! Depcrate_pathimpl_275 {
() => {
// Module: crate::path
// Provides: {"impl_275"}
// Dependencies: {}
impl Debug for CGPathElement { fn fmt (& self , formatter : & mut Formatter) -> Result < () , fmt :: Error > { write ! (formatter , "{:?}: {:?}" , self . element_type , self . points ()) } }
};
}
