// Generated macro for as_micros (function)
macro_rules! Depcrateas_micros {
() => {
// Module: crate
// Provides: {"as_micros"}
// Dependencies: {}
fn as_micros < S : Serializer > (d : & Duration , s : S) -> Result < S :: Ok , S :: Error > { let v = (d . as_secs () * 1_000_000) + (d . subsec_nanos () as u64 / 1_000) ; s . serialize_u64 (v) }
};
}
