// Generated macro for impl_310 (impl)
macro_rules! Depcrate_types_cpp_enumimpl_310 {
() => {
// Module: crate::types::cpp_enum
// Provides: {"impl_310"}
// Dependencies: {}
impl Dependencies for CppEnum { fn combine (& self , dependencies : & mut TypeMap) { if let Some (attribute) = self . def . find_attribute ("AlsoUsableForAttribute") { if let Some ((_ , Value :: Str (type_name))) = attribute . args () . first () { self . def . reader () . unwrap_full_name (self . def . namespace () , type_name) . combine (dependencies) ; } } } }
};
}
