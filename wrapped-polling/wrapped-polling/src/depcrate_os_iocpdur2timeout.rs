// Generated macro for dur2timeout (function)
macro_rules! Depcrate_os_iocpdur2timeout {
() => {
// Module: crate::os::iocp
// Provides: {"dur2timeout"}
// Dependencies: {}
fn dur2timeout (dur : Duration) -> u32 { dur . as_secs () . checked_mul (1000) . and_then (| ms | ms . checked_add ((dur . subsec_nanos () as u64) / 1_000_000)) . and_then (| ms | { if dur . subsec_nanos () % 1_000_000 > 0 { ms . checked_add (1) } else { Some (ms) } }) . and_then (| x | u32 :: try_from (x) . ok ()) . unwrap_or (INFINITE) }
};
}
