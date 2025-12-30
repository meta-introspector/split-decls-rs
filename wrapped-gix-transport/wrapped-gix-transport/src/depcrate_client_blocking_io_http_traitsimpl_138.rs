// Generated macro for impl_138 (impl)
macro_rules! Depcrate_client_blocking_io_http_traitsimpl_138 {
() => {
// Module: crate::client::blocking_io::http::traits
// Provides: {"impl_138"}
// Dependencies: {}
impl < A , B , C > From < PostResponse < A , B , C > > for GetResponse < A , B > { fn from (v : PostResponse < A , B , C >) -> Self { GetResponse { headers : v . headers , body : v . body , } } }
};
}
