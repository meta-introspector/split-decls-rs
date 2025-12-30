// Generated macro for checkpoint (function)
macro_rules! Depcrate_recovery_congestion_cubiccheckpoint {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"checkpoint"}
// Dependencies: {}
fn checkpoint (r : & mut Congestion) { r . cubic_state . prior . congestion_window = r . congestion_window ; r . cubic_state . prior . ssthresh = r . ssthresh . get () ; r . cubic_state . prior . w_max = r . cubic_state . w_max ; r . cubic_state . prior . k = r . cubic_state . k ; r . cubic_state . prior . epoch_start = r . congestion_recovery_start_time ; r . cubic_state . prior . lost_count = r . lost_count ; }
};
}
