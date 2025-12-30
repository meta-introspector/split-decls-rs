// Generated macro for impl_468 (impl)
macro_rules! Depcrate_connection_spacesimpl_468 {
() => {
// Module: crate::connection::spaces
// Provides: {"impl_468"}
// Dependencies: {}
impl :: std :: ops :: BitOrAssign for Retransmits { fn bitor_assign (& mut self , rhs : Self) { self . max_data |= rhs . max_data ; for dir in Dir :: iter () { self . max_stream_id [dir as usize] |= rhs . max_stream_id [dir as usize] ; } self . reset_stream . extend_from_slice (& rhs . reset_stream) ; self . stop_sending . extend_from_slice (& rhs . stop_sending) ; self . max_stream_data . extend (& rhs . max_stream_data) ; for crypto in rhs . crypto . into_iter () . rev () { self . crypto . push_front (crypto) ; } self . new_cids . extend (& rhs . new_cids) ; self . retire_cids . extend (rhs . retire_cids) ; self . ack_frequency |= rhs . ack_frequency ; self . handshake_done |= rhs . handshake_done ; self . new_tokens . extend_from_slice (& rhs . new_tokens) ; } }
};
}
