// Generated macro for rollback (function)
macro_rules! Depcrate_recovery_congestion_cubicrollback {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"rollback"}
// Dependencies: {}
fn rollback (r : & mut Congestion) -> bool { if r . cubic_state . prior . congestion_window < r . cubic_state . prior . ssthresh { return false ; } if r . congestion_window >= r . cubic_state . prior . congestion_window { return false ; } r . congestion_window = r . cubic_state . prior . congestion_window ; r . ssthresh . update (r . cubic_state . prior . ssthresh , false) ; r . cubic_state . w_max = r . cubic_state . prior . w_max ; r . cubic_state . k = r . cubic_state . prior . k ; r . congestion_recovery_start_time = r . cubic_state . prior . epoch_start ; true }
};
}
