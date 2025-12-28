macro_rules! deps {
    () => {
        Months!();
        DateTime!();
        TimeZone!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        # [doc = " Add `Months` to `DateTime`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see `checked_add_months` for"] # [doc = " details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if:"] # [doc = " - The resulting date would be out of range."] # [doc = " - The local time at the resulting date does not exist or is ambiguous, for example during a"] # [doc = "   daylight saving time transition."] # [doc = ""] # [doc = " Strongly consider using [`DateTime<Tz>::checked_add_months`] to get an `Option` instead."] impl < Tz : TimeZone > Add < Months > for DateTime < Tz > { type Output = DateTime < Tz > ; fn add (self , rhs : Months) -> Self :: Output { self . checked_add_months (rhs) . expect ("`DateTime + Months` out of range") } }
    };
}

impl_171!();