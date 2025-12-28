macro_rules! deps {
    () => {
        Month!();
    };
}

macro_rules! ParseMonthError {
    () => {
        deps!();
        # [doc = " An error resulting from reading `<Month>` value with `FromStr`."] # [derive (Clone , PartialEq , Eq)] pub struct ParseMonthError { pub (crate) _dummy : () , }
    };
}

ParseMonthError!()