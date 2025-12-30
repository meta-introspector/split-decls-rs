// Generated macro for impl_37 (impl)
macro_rules! Depcrate_time_deltaimpl_37 {
() => {
// Module: crate::time_delta
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (all (feature = "arbitrary" , feature = "std"))] impl arbitrary :: Arbitrary < '_ > for TimeDelta { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < TimeDelta > { const MIN_SECS : i64 = - i64 :: MAX / MILLIS_PER_SEC - 1 ; const MAX_SECS : i64 = i64 :: MAX / MILLIS_PER_SEC ; let secs : i64 = u . int_in_range (MIN_SECS ..= MAX_SECS) ? ; let nanos : i32 = u . int_in_range (0 ..= (NANOS_PER_SEC - 1)) ? ; let duration = TimeDelta { secs , nanos } ; if duration < MIN || duration > MAX { Err (arbitrary :: Error :: IncorrectFormat) } else { Ok (duration) } } }
};
}
