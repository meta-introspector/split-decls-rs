// Generated macro for std_time_to_u64 (function)
macro_rules! Depcrate_sendtostd_time_to_u64 {
() => {
// Module: crate::sendto
// Provides: {"std_time_to_u64"}
// Dependencies: {}
# [cfg (target_os = "linux")] fn std_time_to_u64 (time : & std :: time :: Instant) -> u64 { const NANOS_PER_SEC : u64 = 1_000_000_000 ; const INSTANT_ZERO : std :: time :: Instant = unsafe { std :: mem :: transmute (std :: time :: UNIX_EPOCH) } ; let raw_time = time . duration_since (INSTANT_ZERO) ; let sec = raw_time . as_secs () ; let nsec = raw_time . subsec_nanos () ; sec * NANOS_PER_SEC + nsec as u64 }
};
}
