// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl < I > From < (I , ErrorKind) > for Error < I > { fn from (e : (I , ErrorKind)) -> Self { Self { input : e . 0 , error : e . 1 , } } }
};
}
