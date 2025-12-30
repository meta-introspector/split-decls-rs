// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl Reply for BlockingError { fn into_response (self) -> reply :: Response { http :: Response :: builder () . status (StatusCode :: INTERNAL_SERVER_ERROR) . body (format ! ("Failed to execute synchronous GraphQL request: {}" , self . 0) . into ()) . unwrap_or_else (| e | { unreachable ! ("cannot build `reply::Response` out of `BlockingError`: {e}") }) } }
};
}
