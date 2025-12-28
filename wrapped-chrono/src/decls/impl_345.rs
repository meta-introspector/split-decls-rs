macro_rules! deps {
    () => {
        NaiveDate!();
        Days!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        # [doc = " Add `Days` to `NaiveDate`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_add_days` to get an `Option` instead."] impl Add < Days > for NaiveDate { type Output = NaiveDate ; fn add (self , days : Days) -> Self :: Output { self . checked_add_days (days) . expect ("`NaiveDate + Days` out of range") } }
    };
}

impl_345!()