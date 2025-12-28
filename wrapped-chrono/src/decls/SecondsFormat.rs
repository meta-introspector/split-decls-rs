macro_rules! SecondsFormat {
    () => {
        # [doc = " Specific formatting options for seconds. This may be extended in the"] # [doc = " future, so exhaustive matching in external code is not recommended."] # [doc = ""] # [doc = " See the `TimeZone::to_rfc3339_opts` function for usage."] # [derive (Clone , Copy , Debug , Eq , PartialEq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] # [allow (clippy :: manual_non_exhaustive)] pub enum SecondsFormat { # [doc = " Format whole seconds only, with no decimal point nor subseconds."] Secs , # [doc = " Use fixed 3 subsecond digits. This corresponds to [Fixed::Nanosecond3]."] Millis , # [doc = " Use fixed 6 subsecond digits. This corresponds to [Fixed::Nanosecond6]."] Micros , # [doc = " Use fixed 9 subsecond digits. This corresponds to [Fixed::Nanosecond9]."] Nanos , # [doc = " Automatically select one of `Secs`, `Millis`, `Micros`, or `Nanos` to display all available"] # [doc = " non-zero sub-second digits.  This corresponds to [Fixed::Nanosecond]."] AutoSi , # [doc (hidden)] __NonExhaustive , }
    };
}

SecondsFormat!();