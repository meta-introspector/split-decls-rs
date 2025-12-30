// Generated macro for impl_348 (impl)
macro_rules! Depcrate_types_cpp_structimpl_348 {
() => {
// Module: crate::types::cpp_struct
// Provides: {"impl_348"}
// Dependencies: {}
impl Dependencies for CppStruct { fn combine (& self , dependencies : & mut TypeMap) { for field in self . def . fields () { field . ty (Some (self)) . combine (dependencies) ; } if let Some (attribute) = self . def . find_attribute ("AlsoUsableForAttribute") { if let Some ((_ , Value :: Str (type_name))) = attribute . args () . first () { self . def . reader () . unwrap_full_name (self . def . namespace () , type_name) . combine (dependencies) ; } } } }
};
}
