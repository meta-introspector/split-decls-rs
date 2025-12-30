// Generated macro for writable_response_streams (function)
macro_rules! Depcrate_commonwritable_response_streams {
() => {
// Module: crate::common
// Provides: {"writable_response_streams"}
// Dependencies: {}
pub fn writable_response_streams (conn : & quiche :: Connection ,) -> impl Iterator < Item = u64 > { conn . writable () . filter (| id | id % 4 == 0) }
};
}
