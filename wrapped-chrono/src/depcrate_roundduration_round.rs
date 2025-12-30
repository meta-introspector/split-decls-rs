// Generated macro for duration_round (function)
macro_rules! Depcrate_roundduration_round {
() => {
// Module: crate::round
// Provides: {"duration_round"}
// Dependencies: {}
fn duration_round < T > (naive : NaiveDateTime , original : T , duration : TimeDelta ,) -> Result < T , RoundingError > where T : Timelike + Add < TimeDelta , Output = T > + Sub < TimeDelta , Output = T > , { if let Some (span) = duration . num_nanoseconds () { if span <= 0 { return Err (RoundingError :: DurationExceedsLimit) ; } let stamp = naive . and_utc () . timestamp_nanos_opt () . ok_or (RoundingError :: TimestampExceedsLimit) ? ; let delta_down = stamp % span ; if delta_down == 0 { Ok (original) } else { let (delta_up , delta_down) = if delta_down < 0 { (delta_down . abs () , span - delta_down . abs ()) } else { (span - delta_down , delta_down) } ; if delta_up <= delta_down { Ok (original + TimeDelta :: nanoseconds (delta_up)) } else { Ok (original - TimeDelta :: nanoseconds (delta_down)) } } } else { Err (RoundingError :: DurationExceedsLimit) } }
};
}
