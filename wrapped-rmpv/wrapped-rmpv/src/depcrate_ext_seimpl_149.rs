// Generated macro for impl_149 (impl)
macro_rules! Depcrate_ext_seimpl_149 {
() => {
// Module: crate::ext::se
// Provides: {"impl_149"}
// Dependencies: {}
impl ExtSerializer { # [inline] const fn new () -> Self { Self { fields_se : None } } fn value (self) -> Result < Value , Error > { match self . fields_se { Some (fields_se) => fields_se . value () , None => Err (< Error as ser :: Error > :: custom ("expected tuple")) } } }
};
}
