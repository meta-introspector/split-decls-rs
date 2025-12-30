// Generated macro for add_current (function)
macro_rules! Depcrate_durationadd_current {
() => {
// Module: crate::duration
// Provides: {"add_current"}
// Dependencies: {}
fn add_current (mut sec : u64 , nsec : u64 , out : & mut Duration) -> Result < () , Error > { let mut nsec = (out . subsec_nanos () as u64) . add (nsec) ? ; if nsec > 1_000_000_000 { sec = sec . add (nsec / 1_000_000_000) ? ; nsec %= 1_000_000_000 ; } sec = out . as_secs () . add (sec) ? ; * out = Duration :: new (sec , nsec as u32) ; Ok (()) }
};
}
