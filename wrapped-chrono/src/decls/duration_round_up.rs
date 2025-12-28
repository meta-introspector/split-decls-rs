macro_rules! deps {
    () => {
        NaiveDateTime!();
        TimeDelta!();
        Timelike!();
        RoundingError!();
    };
}

macro_rules! duration_round_up {
    () => {
        deps!();
        fn duration_round_up < T > (naive : NaiveDateTime , original : T , duration : TimeDelta ,) -> Result < T , RoundingError > where T : Timelike + Add < TimeDelta , Output = T > + Sub < TimeDelta , Output = T > , { if let Some (span) = duration . num_nanoseconds () { if span <= 0 { return Err (RoundingError :: DurationExceedsLimit) ; } let stamp = naive . and_utc () . timestamp_nanos_opt () . ok_or (RoundingError :: TimestampExceedsLimit) ? ; let delta_down = stamp % span ; match delta_down . cmp (& 0) { Ordering :: Equal => Ok (original) , Ordering :: Greater => Ok (original + TimeDelta :: nanoseconds (span - delta_down)) , Ordering :: Less => Ok (original + TimeDelta :: nanoseconds (delta_down . abs ())) , } } else { Err (RoundingError :: DurationExceedsLimit) } }
    };
}

duration_round_up!()