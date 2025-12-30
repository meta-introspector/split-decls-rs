// Generated macro for impl_357 (impl)
macro_rules! Depcrate_clockimpl_357 {
() => {
// Module: crate::clock
// Provides: {"impl_357"}
// Dependencies: {}
impl Instant { # [doc = " Will try to add `duration`, but if that overflows it may add less."] pub fn add_lossy (& self , duration : Duration) -> Instant { match self . kind { InstantKind :: Host (instant) => { let i = instant . checked_add (duration) . unwrap_or_else (| | instant . checked_add (Duration :: from_secs (3600)) . unwrap ()) ; Instant { kind : InstantKind :: Host (i) } } InstantKind :: Virtual { nanoseconds } => { let n = nanoseconds . saturating_add (duration . as_nanos ()) ; Instant { kind : InstantKind :: Virtual { nanoseconds : n } } } } } pub fn duration_since (& self , earlier : Instant) -> Duration { match (& self . kind , earlier . kind) { (InstantKind :: Host (instant) , InstantKind :: Host (earlier)) => instant . duration_since (earlier) , (InstantKind :: Virtual { nanoseconds } , InstantKind :: Virtual { nanoseconds : earlier } ,) => { let duration = nanoseconds . saturating_sub (earlier) ; let seconds = u64 :: try_from (duration / 1_000_000_000) . unwrap_or (u64 :: MAX) ; let nanosecond = u32 :: try_from (duration . wrapping_rem (1_000_000_000)) . unwrap () ; Duration :: new (seconds , nanosecond) } _ => panic ! ("all `Instant` must be of the same kind") , } } }
};
}
