// Generated macro for now (function)
macro_rules! Depcratenow {
() => {
// Module: crate
// Provides: {"now"}
// Dependencies: {}
# [doc = " Gets the current time as an epoch timestamp, caching it so future calls return the same time."] # [allow (clippy :: missing_panics_doc)] # [must_use] pub fn now () -> u64 { static CACHED_VALUE : AtomicU64 = AtomicU64 :: new (0) ; let value = CACHED_VALUE . load (Ordering :: Acquire) ; if value != 0 { return value ; } let value = std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs () ; CACHED_VALUE . store (value , Ordering :: Release) ; value }
};
}
