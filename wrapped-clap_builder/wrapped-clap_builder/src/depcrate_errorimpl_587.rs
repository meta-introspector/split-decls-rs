// Generated macro for impl_587 (impl)
macro_rules! Depcrate_errorimpl_587 {
() => {
// Module: crate::error
// Provides: {"impl_587"}
// Dependencies: {}
impl < F : ErrorFormatter > error :: Error for Error < F > { # [allow (trivial_casts)] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { self . inner . source . as_ref () . map (| e | e . as_ref () as _) } }
};
}
