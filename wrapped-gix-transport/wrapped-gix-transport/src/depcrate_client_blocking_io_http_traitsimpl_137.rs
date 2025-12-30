// Generated macro for impl_137 (impl)
macro_rules! Depcrate_client_blocking_io_http_traitsimpl_137 {
() => {
// Module: crate::client::blocking_io::http::traits
// Provides: {"impl_137"}
// Dependencies: {}
impl From < WriteMode > for PostBodyDataKind { fn from (m : WriteMode) -> Self { match m { WriteMode :: Binary => PostBodyDataKind :: Unbounded , WriteMode :: OneLfTerminatedLinePerWriteCall => PostBodyDataKind :: BoundedAndFitsIntoMemory , } } }
};
}
