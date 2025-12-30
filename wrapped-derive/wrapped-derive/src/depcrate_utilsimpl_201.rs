// Generated macro for impl_201 (impl)
macro_rules! Depcrate_utilsimpl_201 {
() => {
// Module: crate::utils
// Provides: {"impl_201"}
// Dependencies: {}
impl GeneratorError { pub fn write_errors (self) -> TokenStream { match self { GeneratorError :: Syn (err) => err . to_compile_error () , GeneratorError :: Darling (err) => err . write_errors () , } } }
};
}
