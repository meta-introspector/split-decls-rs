// Generated macro for get_or_insert_send (function)
macro_rules! Depcrate_connection_streams_stateget_or_insert_send {
() => {
// Module: crate::connection::streams::state
// Provides: {"get_or_insert_send"}
// Dependencies: {}
# [inline] pub (super) fn get_or_insert_send (max_data : VarInt ,) -> impl Fn (& mut Option < Box < Send > >) -> & mut Box < Send > { move | opt | opt . get_or_insert_with (| | Send :: new (max_data)) }
};
}
