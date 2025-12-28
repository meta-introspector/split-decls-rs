macro_rules! deps {
    () => {
        Days!();
        NaiveDate!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        # [doc = " Subtract `Days` from `NaiveDate`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_sub_days` to get an `Option` instead."] impl Sub < Days > for NaiveDate { type Output = NaiveDate ; fn sub (self , days : Days) -> Self :: Output { self . checked_sub_days (days) . expect ("`NaiveDate - Days` out of range") } }
    };
}

impl_346!()