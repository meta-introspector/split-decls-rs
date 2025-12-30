// Generated macro for NoticeProcessor (type)
macro_rules! Depcrate_pg_connection_rawNoticeProcessor {
() => {
// Module: crate::pg::connection::raw
// Provides: {"NoticeProcessor"}
// Dependencies: {}
pub (super) type NoticeProcessor = extern "C" fn (arg : * mut libc :: c_void , message : * const libc :: c_char) ;
};
}
