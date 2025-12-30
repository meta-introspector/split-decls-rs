// Generated macro for DownloadProgressRecord (struct)
macro_rules! DepcrateDownloadProgressRecord {
() => {
// Module: crate
// Provides: {"DownloadProgressRecord"}
// Dependencies: {}
# [doc = " Structure modeling information about download progress"] # [derive (Debug)] pub struct DownloadProgressRecord { pub elapsed_time : Duration , pub last_elapsed_time : Duration , pub last_throughput : f32 , pub total_throughput : f32 , pub total_bytes : usize , pub current_bytes : usize , pub percentage_done : f32 , pub estimated_remaining_time : f32 , pub notification_count : u64 , }
};
}
