// Generated macro for print_stats (function)
macro_rules! Depcrate_reports_textprint_stats {
() => {
// Module: crate::reports::text
// Provides: {"print_stats"}
// Dependencies: {}
pub fn print_stats (data_store : & Datastore , config : & PrintStatsConfig) { if config . rx_flow_control { print_rx_max_data_frames (data_store) ; print_rx_max_stream_data_frames (data_store) ; } if config . tx_flow_control { print_tx_max_data_frames (data_store) ; print_tx_max_stream_data_frames (data_store) ; } if config . reset_streams { print_tx_reset_stream_frames (data_store) ; print_rx_reset_stream_frames (data_store) ; } if config . tx_stream_frames { print_tx_stream_frames (data_store) ; } if config . stream_buffering { print_local_stream_buffer_reads (data_store) ; print_local_stream_buffer_writes (data_store) ; print_local_stream_buffer_dropped (data_store) ; } if config . packet_stats { print_sent_packet_stats (data_store) ; } }
};
}
