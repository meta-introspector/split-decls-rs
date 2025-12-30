// Generated macro for impl_94 (impl)
macro_rules! Depcrate_arrayvecimpl_94 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_94"}
// Dependencies: {}
impl < T , const CAP : usize > fmt :: Debug for IntoIter < T , CAP > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (& self . v [self . index ..]) . finish () } }
};
}
