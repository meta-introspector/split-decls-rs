// Generated macro for CommonArgs (struct)
macro_rules! Depcrate_argsCommonArgs {
() => {
// Module: crate::args
// Provides: {"CommonArgs"}
// Dependencies: {}
# [doc = " Contains commons arguments for creating a quiche QUIC connection."] pub struct CommonArgs { pub alpns : Vec < & 'static [u8] > , pub max_data : u64 , pub max_window : u64 , pub max_stream_data : u64 , pub max_stream_window : u64 , pub max_streams_bidi : u64 , pub max_streams_uni : u64 , pub idle_timeout : u64 , pub early_data : bool , pub dump_packet_path : Option < String > , pub no_grease : bool , pub cc_algorithm : String , pub disable_hystart : bool , pub dgrams_enabled : bool , pub dgram_count : u64 , pub dgram_data : String , pub max_active_cids : u64 , pub enable_active_migration : bool , pub max_field_section_size : Option < u64 > , pub qpack_max_table_capacity : Option < u64 > , pub qpack_blocked_streams : Option < u64 > , pub initial_rtt : Duration , pub initial_cwnd_packets : u64 , }
};
}
