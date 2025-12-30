// Generated macro for ext (module)
macro_rules! Depcrate_contextext {
() => {
// Module: crate::context
// Provides: {"ext"}
// Dependencies: {}
mod ext { use super :: * ; pub trait StdError { # [cfg_attr (track_caller , track_caller)] fn ext_report < D > (self , msg : D) -> Report where D : Display + Send + Sync + 'static ; } impl < E > StdError for E where E : std :: error :: Error + Send + Sync + 'static , { fn ext_report < D > (self , msg : D) -> Report where D : Display + Send + Sync + 'static , { Report :: from_msg (msg , self) } } impl StdError for Report { fn ext_report < D > (self , msg : D) -> Report where D : Display + Send + Sync + 'static , { self . wrap_err (msg) } } }
};
}
