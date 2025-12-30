// Generated macro for is_id_start (function)
macro_rules! Depcrateis_id_start {
() => {
// Module: crate
// Provides: {"is_id_start"}
// Dependencies: {}
fn is_id_start (c : char) -> bool { c == '_' || unicode_xid :: UnicodeXID :: is_xid_start (c) }
};
}
