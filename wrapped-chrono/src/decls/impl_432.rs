macro_rules! deps {
    () => {
        NaiveDateTime!();
        Days!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        # [doc = " Add `Days` to `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_add_days` to get an `Option` instead."] impl Add < Days > for NaiveDateTime { type Output = NaiveDateTime ; fn add (self , days : Days) -> Self :: Output { self . checked_add_days (days) . expect ("`NaiveDateTime + Days` out of range") } }
    };
}

impl_432!()