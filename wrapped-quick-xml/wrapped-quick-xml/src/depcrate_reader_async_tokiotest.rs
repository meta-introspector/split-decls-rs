// Generated macro for test (module)
macro_rules! Depcrate_reader_async_tokiotest {
() => {
// Module: crate::reader::async_tokio
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: TokioAdapter ; use crate :: reader :: test :: check ; check ! (# [tokio :: test] read_event_into_async , read_until_close_async , TokioAdapter , & mut Vec :: new () , async , await) ; # [test] fn test_future_is_send () { use super :: * ; use tokio :: io :: BufReader ; fn check_send < T : Send > (_ : T) { } let input = vec ! [] ; let mut reading_buf = vec ! [] ; let mut reader = Reader :: from_reader (BufReader :: new (input . as_slice ())) ; check_send (reader . read_event_into_async (& mut reading_buf)) ; } }
};
}
