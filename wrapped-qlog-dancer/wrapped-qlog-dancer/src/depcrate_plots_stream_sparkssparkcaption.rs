// Generated macro for SparkCaption (enum)
macro_rules! Depcrate_plots_stream_sparksSparkCaption {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"SparkCaption"}
// Dependencies: {}
# [derive (Tabled)] enum SparkCaption { UniStream { summary : String , } , RequestAtServer { summary : String , method : String , path : String , client_content_length : String , server_content_length : String , client_pri_hdr : String , server_pri_hdr : String , duration_rx_hdr_tx_hdr : String , duration_rx_hdr_tx_first_data : String , duration_rx_hdr_tx_last_data : String , duration_tx_first_data_tx_last_data : String , } , RequestAtClient { summary : String , method : String , path : String , client_content_length : String , server_content_length : String , client_pri_hdr : String , client_pri_update : String , server_pri_hdr : String , duration_tx_hdr_rx_hdr : String , duration_tx_hdr_rx_first_data : String , duration_tx_hdr_rx_last_data : String , duration_tx_first_data_tx_last_data : String , } , }
};
}
