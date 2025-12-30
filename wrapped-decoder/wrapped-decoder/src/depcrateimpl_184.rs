// Generated macro for impl_184 (impl)
macro_rules! Depcrateimpl_184 {
() => {
// Module: crate
// Provides: {"impl_184"}
// Dependencies: {}
impl From < io :: Error > for DecodeError { fn from (e : io :: Error) -> Self { if e . kind () == io :: ErrorKind :: UnexpectedEof { DecodeError :: UnexpectedEof } else { DecodeError :: Malformed } } }
};
}
