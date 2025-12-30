// Generated macro for impl_1065 (impl)
macro_rules! Depcrate_runtime_mallocimpl_1065 {
() => {
// Module: crate::runtime::malloc
// Provides: {"impl_1065"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for MallocSlice < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
