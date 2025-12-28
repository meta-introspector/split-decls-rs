macro_rules! deps {
    () => {
        Mdf!();
        Weekday!();
        IsoWeek!();
        NaiveDate!();
    };
}

macro_rules! YearFlags {
    () => {
        deps!();
        # [doc = " Year flags (aka the dominical letter)."] # [doc = ""] # [doc = " `YearFlags` are used as the last four bits of `NaiveDate`, `Mdf` and `IsoWeek`."] # [doc = ""] # [doc = " There are 14 possible classes of year in the Gregorian calendar:"] # [doc = " common and leap years starting with Monday through Sunday."] # [doc = ""] # [doc = " The `YearFlags` stores this information into 4 bits `LWWW`. `L` is the leap year flag, with `1`"] # [doc = " for the common year (this simplifies validating an ordinal in `NaiveDate`). `WWW` is a non-zero"] # [doc = " `Weekday` of the last day in the preceding year."] # [allow (unreachable_pub)] # [derive (PartialEq , Eq , Copy , Clone , Hash)] pub struct YearFlags (pub (super) u8) ;
    };
}

YearFlags!();