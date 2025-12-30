// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl From < GlobError > for std :: io :: Error { fn from (e : GlobError) -> Self { if let ignore :: Error :: Io (e) = e . 0 { e } else { std :: io :: ErrorKind :: Other . into () } } }
};
}
