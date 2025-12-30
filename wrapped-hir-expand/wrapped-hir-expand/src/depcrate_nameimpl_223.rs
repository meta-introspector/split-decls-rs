// Generated macro for impl_223 (impl)
macro_rules! Depcrate_nameimpl_223 {
() => {
// Module: crate::name
// Provides: {"impl_223"}
// Dependencies: {}
impl AsName for ast :: NameRef { fn as_name (& self) -> Name { match self . as_tuple_field () { Some (idx) => Name :: new_tuple_field (idx) , None => Name :: new_root (& self . text ()) , } } }
};
}
