macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDate!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        # [doc = " Add-assign of `TimeDelta` to `NaiveDate`."] # [doc = ""] # [doc = " This discards the fractional days in `TimeDelta`, rounding to the closest integral number of days"] # [doc = " towards `TimeDelta::zero()`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDate::checked_add_signed`] to get an `Option` instead."] impl AddAssign < TimeDelta > for NaiveDate { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { * self = self . add (rhs) ; } }
    };
}

impl_342!()