// Generated macro for impl_467 (impl)
macro_rules! Depcrate_connection_spacesimpl_467 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_467"}
// Dependencies: {}
impl Retransmits { pub (super) fn is_empty (& self , streams : & StreamsState) -> bool { ! self . max_data && ! self . max_stream_id . into_iter () . any (| x | x) && self . reset_stream . is_empty () && self . stop_sending . is_empty () && self . max_stream_data . iter () . all (| & id | ! streams . can_send_flow_control (id)) && self . crypto . is_empty () && self . new_cids . is_empty () && self . retire_cids . is_empty () && ! self . ack_frequency && ! self . handshake_done && self . new_tokens . is_empty () } }
};
}
