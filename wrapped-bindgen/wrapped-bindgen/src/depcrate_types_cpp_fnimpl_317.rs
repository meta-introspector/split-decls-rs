// Generated macro for impl_317 (impl)
macro_rules! Depcrate_types_cpp_fnimpl_317 {
() => {
// Module: crate::types::cpp_fn
// Provides: {"impl_317"}
// Dependencies: {}
impl Dependencies for CppFn { fn combine (& self , dependencies : & mut TypeMap) { self . method . signature (self . namespace , & []) . combine (dependencies) ; let dependency = match self . method . name () { "GetWindowLongPtrA" => Some ("GetWindowLongA") , "GetWindowLongPtrW" => Some ("GetWindowLongW") , "SetWindowLongPtrA" => Some ("SetWindowLongA") , "SetWindowLongPtrW" => Some ("SetWindowLongW") , _ => None , } ; if let Some (dependency) = dependency { self . method . reader () . unwrap_full_name (self . namespace , dependency) . combine (dependencies) ; } } }
};
}
