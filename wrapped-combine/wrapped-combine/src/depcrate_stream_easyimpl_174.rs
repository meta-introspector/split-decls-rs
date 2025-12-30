// Generated macro for impl_174 (impl)
macro_rules! Depcrate_stream_easyimpl_174 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_174"}
// Dependencies: {}
impl < T , R , E > From < E > for Error < T , R > where E : StdError + 'static + Send + Sync , { fn from (e : E) -> Error < T , R > { Error :: Other (Box :: new (e)) } }
};
}
