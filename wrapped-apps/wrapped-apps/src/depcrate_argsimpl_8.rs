// Generated macro for impl_8 (impl)
macro_rules! Depcrate_argsimpl_8 {
() => {
// Module: crate::args
// Provides: {"impl_8"}
// Dependencies: {}
impl Default for CommonArgs { fn default () -> Self { CommonArgs { alpns : alpns :: HTTP_3 . to_vec () , max_data : 10000000 , max_window : 25165824 , max_stream_data : 1000000 , max_stream_window : 16777216 , max_streams_bidi : 100 , max_streams_uni : 100 , idle_timeout : 30000 , early_data : false , dump_packet_path : None , no_grease : false , cc_algorithm : "cubic" . to_string () , disable_hystart : false , dgrams_enabled : false , dgram_count : 0 , dgram_data : "quack" . to_string () , max_active_cids : 2 , enable_active_migration : false , max_field_section_size : None , qpack_max_table_capacity : None , qpack_blocked_streams : None , initial_rtt : Duration :: from_millis (333) , initial_cwnd_packets : 10 , } } }
};
}
