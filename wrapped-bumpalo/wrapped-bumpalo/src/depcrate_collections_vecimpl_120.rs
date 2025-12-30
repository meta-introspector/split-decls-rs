// Generated macro for impl_120 (impl)
macro_rules! Depcrate_collections_vecimpl_120 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'bump , T : 'bump + fmt :: Debug > fmt :: Debug for Vec < 'bump , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
