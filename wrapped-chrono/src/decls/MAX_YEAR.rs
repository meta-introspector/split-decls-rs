macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! MAX_YEAR {
    () => {
        deps!();
        # [doc = " MAX_YEAR is one year less than the type is capable of representing. Internally we may sometimes"] # [doc = " use the headroom, notably to handle cases where the offset of a `DateTime` constructed with"] # [doc = " `NaiveDate::MAX` pushes it beyond the valid, representable range."] pub (super) const MAX_YEAR : i32 = (i32 :: MAX >> 13) - 1 ;
    };
}

MAX_YEAR!();