// Generated macro for impl_327 (impl)
macro_rules! Depcrate_types_cpp_interfaceimpl_327 {
() => {
// Module: crate::types::cpp_interface
// Provides: {"impl_327"}
// Dependencies: {}
impl Dependencies for CppInterface { fn combine (& self , dependencies : & mut TypeMap) { let base_interfaces = self . base_interfaces () ; for interface in & base_interfaces { interface . combine (dependencies) ; } for method in self . def . methods () { for ty in method . signature (self . def . namespace () , & []) . types () { if ty . is_core () { ty . combine (dependencies) ; } } } } }
};
}
