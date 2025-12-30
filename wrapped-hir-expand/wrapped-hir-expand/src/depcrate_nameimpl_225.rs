// Generated macro for impl_225 (impl)
macro_rules! Depcrate_nameimpl_225 {
() => {
// Module: crate::name
// Provides: {"impl_225"}
// Dependencies: {}
impl AsName for ast :: NameOrNameRef { fn as_name (& self) -> Name { match self { ast :: NameOrNameRef :: Name (it) => it . as_name () , ast :: NameOrNameRef :: NameRef (it) => it . as_name () , } } }
};
}
