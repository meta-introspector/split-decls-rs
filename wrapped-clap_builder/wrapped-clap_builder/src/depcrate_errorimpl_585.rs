// Generated macro for impl_585 (impl)
macro_rules! Depcrate_errorimpl_585 {
() => {
// Module: crate::error
// Provides: {"impl_585"}
// Dependencies: {}
impl < F : ErrorFormatter > From < fmt :: Error > for Error < F > { fn from (e : fmt :: Error) -> Self { Error :: raw (ErrorKind :: Format , e) } }
};
}
