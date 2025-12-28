macro_rules! deps {
    () => {
        Weekday!();
    };
}

macro_rules! ParseWeekdayError {
    () => {
        deps!();
        # [doc = " An error resulting from reading `Weekday` value with `FromStr`."] # [derive (Clone , PartialEq , Eq)] pub struct ParseWeekdayError { pub (crate) _dummy : () , }
    };
}

ParseWeekdayError!();