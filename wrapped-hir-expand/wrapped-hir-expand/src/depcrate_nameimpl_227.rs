// Generated macro for impl_227 (impl)
macro_rules! Depcrate_nameimpl_227 {
() => {
// Module: crate::name
// Provides: {"impl_227"}
// Dependencies: {}
impl AsName for ast :: FieldKind { fn as_name (& self) -> Name { match self { ast :: FieldKind :: Name (nr) => nr . as_name () , ast :: FieldKind :: Index (idx) => { let idx = idx . text () . parse :: < usize > () . unwrap_or (0) ; Name :: new_tuple_field (idx) } } } }
};
}
