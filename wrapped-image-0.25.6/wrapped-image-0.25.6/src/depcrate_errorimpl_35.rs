// Generated macro for impl_35 (impl)
macro_rules! Depcrate_errorimpl_35 {
() => {
// Module: crate::error
// Provides: {"impl_35"}
// Dependencies: {}
impl From < io :: Error > for ImageError { fn from (err : io :: Error) -> ImageError { ImageError :: IoError (err) } }
};
}
