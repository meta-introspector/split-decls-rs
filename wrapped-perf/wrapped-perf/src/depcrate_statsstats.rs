// Generated macro for Stats (struct)
macro_rules! Depcrate_statsStats {
() => {
// Module: crate::stats
// Provides: {"Stats"}
// Dependencies: {}
pub struct Stats { # [doc = " Test start time"] start_instant : Instant , # [doc = " Test start system time"] start : SystemTime , # [doc = " Durations of uploads"] upload_duration : Histogram < u64 > , # [doc = " Durations of downloads"] download_duration : Histogram < u64 > , # [doc = " Time from finishing the upload until receiving the first byte of the response"] fbl : Histogram < u64 > , # [doc = " Throughput for uploads"] upload_throughput : Histogram < u64 > , # [doc = " Throughput for downloads"] download_throughput : Histogram < u64 > , # [doc = " The total amount of requests executed"] requests : usize , # [doc = " Stats accumulated over each interval"] intervals : Vec < Interval > , }
};
}
