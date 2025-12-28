macro_rules! deps {
    () => {
        Days!();
        TimeZone!();
        DateTime!();
        NaiveDateTime!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        # [doc = " Add `Days` to `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if:"] # [doc = " - The resulting date would be out of range."] # [doc = " - The local time at the resulting date does not exist or is ambiguous, for example during a"] # [doc = "   daylight saving time transition."] # [doc = ""] # [doc = " Strongly consider using `DateTime<Tz>::checked_add_days` to get an `Option` instead."] impl < Tz : TimeZone > Add < Days > for DateTime < Tz > { type Output = DateTime < Tz > ; fn add (self , days : Days) -> Self :: Output { self . checked_add_days (days) . expect ("`DateTime + Days` out of range") } }
    };
}

impl_180!();