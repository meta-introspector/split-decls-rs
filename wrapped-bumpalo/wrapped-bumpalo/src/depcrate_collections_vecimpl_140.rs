// Generated macro for impl_140 (impl)
macro_rules! Depcrate_collections_vecimpl_140 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , 'bump , T : 'a + 'bump + fmt :: Debug > fmt :: Debug for Drain < 'a , 'bump , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
};
}
