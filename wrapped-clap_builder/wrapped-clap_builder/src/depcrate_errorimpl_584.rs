// Generated macro for impl_584 (impl)
macro_rules! Depcrate_errorimpl_584 {
() => {
// Module: crate::error
// Provides: {"impl_584"}
// Dependencies: {}
impl < F : ErrorFormatter > From < io :: Error > for Error < F > { fn from (e : io :: Error) -> Self { Error :: raw (ErrorKind :: Io , e) } }
};
}
