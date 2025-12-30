// Generated macro for impl_130 (impl)
macro_rules! Depcrate_collections_vecimpl_130 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'bump , T : fmt :: Debug > fmt :: Debug for IntoIter < 'bump , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
};
}
