// Generated macro for impl_119 (impl)
macro_rules! Depcrate_setimpl_119 {
() => {
// Module: crate::set
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'a , T > fmt :: Debug for Iter < 'a , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
