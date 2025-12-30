// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl std :: error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Error :: CouldNotRead { source , .. } => Some (source) , Error :: InvalidToml { source } => Some (source) , _ => None , } } }
};
}
