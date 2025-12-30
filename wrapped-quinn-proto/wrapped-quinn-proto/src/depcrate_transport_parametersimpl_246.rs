// Generated macro for impl_246 (impl)
macro_rules! Depcrate_transport_parametersimpl_246 {
() => {
// Module: crate::transport_parameters
// Provides: {"impl_246"}
// Dependencies: {}
impl From < Error > for TransportError { fn from (e : Error) -> Self { match e { Error :: IllegalValue => Self :: TRANSPORT_PARAMETER_ERROR ("illegal value") , Error :: Malformed => Self :: TRANSPORT_PARAMETER_ERROR ("malformed") , } } }
};
}
