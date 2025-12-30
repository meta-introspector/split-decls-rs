// Generated macro for impl_286 (impl)
macro_rules! Depcrate_types_classimpl_286 {
() => {
// Module: crate::types::class
// Provides: {"impl_286"}
// Dependencies: {}
impl Dependencies for Class { fn combine (& self , dependencies : & mut TypeMap) { for interface in self . required_interfaces () { Type :: Interface (interface) . combine (dependencies) ; } } }
};
}
