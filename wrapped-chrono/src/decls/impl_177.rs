macro_rules! deps {
    () => {
        DateTime!();
        Months!();
        TimeZone!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        # [doc = " Subtract `Months` from `DateTime`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see"] # [doc = " [`DateTime<Tz>::checked_sub_months`] for details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if:"] # [doc = " - The resulting date would be out of range."] # [doc = " - The local time at the resulting date does not exist or is ambiguous, for example during a"] # [doc = "   daylight saving time transition."] # [doc = ""] # [doc = " Strongly consider using [`DateTime<Tz>::checked_sub_months`] to get an `Option` instead."] impl < Tz : TimeZone > Sub < Months > for DateTime < Tz > { type Output = DateTime < Tz > ; fn sub (self , rhs : Months) -> Self :: Output { self . checked_sub_months (rhs) . expect ("`DateTime - Months` out of range") } }
    };
}

impl_177!();