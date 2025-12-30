// Generated macro for impl_153 (impl)
macro_rules! Depcrate_errorimpl_153 {
() => {
// Module: crate::error
// Provides: {"impl_153"}
// Dependencies: {}
impl From < TryFromIntError > for Error { fn from (_ : TryFromIntError) -> Error { Error { kind : ErrorKind :: Overflow , position : None , } } }
};
}
