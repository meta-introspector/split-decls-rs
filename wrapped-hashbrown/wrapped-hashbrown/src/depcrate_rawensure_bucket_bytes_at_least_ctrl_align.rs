// Generated macro for ensure_bucket_bytes_at_least_ctrl_align (function)
macro_rules! Depcrate_rawensure_bucket_bytes_at_least_ctrl_align {
() => {
// Module: crate::raw
// Provides: {"ensure_bucket_bytes_at_least_ctrl_align"}
// Dependencies: {}
# [inline] fn ensure_bucket_bytes_at_least_ctrl_align (table_layout : TableLayout , buckets : usize) { if table_layout . size != 0 { let prod = table_layout . size . saturating_mul (buckets) ; debug_assert ! (prod >= table_layout . ctrl_align) ; } }
};
}
