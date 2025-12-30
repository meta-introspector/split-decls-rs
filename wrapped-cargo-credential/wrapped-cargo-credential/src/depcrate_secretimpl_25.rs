// Generated macro for impl_25 (impl)
macro_rules! Depcrate_secretimpl_25 {
() => {
// Module: crate::secret
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , E > Secret < Result < T , E > > { # [doc = " Converts a `Secret<Result<T, E>>` to a `Result<Secret<T>, E>`."] pub fn transpose (self) -> Result < Secret < T > , E > { self . inner . map (| v | Secret :: from (v)) } }
};
}
