// Generated macro for impl_305 (impl)
macro_rules! Depcrate_proto_errorimpl_305 {
() => {
// Module: crate::proto::error
// Provides: {"impl_305"}
// Dependencies: {}
impl From < io :: Error > for Error { fn from (src : io :: Error) -> Self { Error :: Io (src . kind () , src . get_ref () . map (| inner | inner . to_string ())) } }
};
}
