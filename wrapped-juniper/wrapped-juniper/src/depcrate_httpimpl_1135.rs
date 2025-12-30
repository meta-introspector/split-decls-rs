// Generated macro for impl_1135 (impl)
macro_rules! Depcrate_httpimpl_1135 {
() => {
// Module: crate::http
// Provides: {"impl_1135"}
// Dependencies: {}
impl < S : ScalarValue > GraphQLBatchResponse < S > { # [doc = " Returns if all the GraphQLResponse in this operation are ok,"] # [doc = " you can use it to determine wheter to send a 200 or 400 HTTP status code."] pub fn is_ok (& self) -> bool { match self { Self :: Single (resp) => resp . is_ok () , Self :: Batch (resps) => resps . iter () . all (GraphQLResponse :: is_ok) , } } }
};
}
