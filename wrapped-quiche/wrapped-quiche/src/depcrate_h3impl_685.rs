// Generated macro for impl_685 (impl)
macro_rules! Depcrate_h3impl_685 {
() => {
// Module: crate::h3
// Provides: {"impl_685"}
// Dependencies: {}
impl From < super :: Error > for Error { fn from (err : super :: Error) -> Self { match err { super :: Error :: Done => Error :: Done , _ => Error :: TransportError (err) , } } }
};
}
