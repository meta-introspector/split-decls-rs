// Generated macro for impl_99 (impl)
macro_rules! Depcrate_ordered_setimpl_99 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a , T > fmt :: Debug for Iter < 'a , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
