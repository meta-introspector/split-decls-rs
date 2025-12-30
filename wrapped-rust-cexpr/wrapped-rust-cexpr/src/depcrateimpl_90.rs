// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl < I > From < (I , nom :: ErrorKind) > for Error < I > { fn from (e : (I , nom :: ErrorKind)) -> Self { Self :: from ((e . 0 , ErrorKind :: from (e . 1))) } }
};
}
