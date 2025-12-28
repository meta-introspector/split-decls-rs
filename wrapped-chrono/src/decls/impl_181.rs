macro_rules! deps {
    () => {
        Days!();
        DateTime!();
        TimeZone!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        # [doc = " Subtract `Days` from `DateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if:"] # [doc = " - The resulting date would be out of range."] # [doc = " - The local time at the resulting date does not exist or is ambiguous, for example during a"] # [doc = "   daylight saving time transition."] # [doc = ""] # [doc = " Strongly consider using `DateTime<Tz>::checked_sub_days` to get an `Option` instead."] impl < Tz : TimeZone > Sub < Days > for DateTime < Tz > { type Output = DateTime < Tz > ; fn sub (self , days : Days) -> Self :: Output { self . checked_sub_days (days) . expect ("`DateTime - Days` out of range") } }
    };
}

impl_181!();