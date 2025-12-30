// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl < I > :: nom :: error :: ParseError < I > for Error < I > { fn from_error_kind (input : I , kind : nom :: ErrorKind) -> Self { Self { input , error : kind . into () , } } fn append (_ : I , _ : nom :: ErrorKind , other : Self) -> Self { other } }
};
}
