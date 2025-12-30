// Generated macro for impl_1329 (impl)
macro_rules! Depcrate_remote_connection_fetchimpl_1329 {
() => {
// Module: crate::remote::connection::fetch
// Provides: {"impl_1329"}
// Dependencies: {}
impl RefLogMessage { pub (crate) fn compose (& self , context : & str) -> BString { match self { RefLogMessage :: Prefixed { action } => format ! ("{action}: {context}") . into () , RefLogMessage :: Override { message } => message . to_owned () , } } }
};
}
