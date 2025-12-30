// Generated macro for impl_353 (impl)
macro_rules! Depcrate_types_delegateimpl_353 {
() => {
// Module: crate::types::delegate
// Provides: {"impl_353"}
// Dependencies: {}
impl Dependencies for Delegate { fn combine (& self , dependencies : & mut TypeMap) { dependencies . combine (& self . method () . dependencies) ; for ty in & self . generics { ty . combine (dependencies) ; } } }
};
}
