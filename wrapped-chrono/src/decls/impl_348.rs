macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        # [doc = " Subtract-assign `TimeDelta` from `NaiveDate`."] # [doc = ""] # [doc = " This discards the fractional days in `TimeDelta`, rounding to the closest integral number of"] # [doc = " days towards `TimeDelta::zero()`."] # [doc = " It is the same as the addition with a negated `TimeDelta`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDate::checked_sub_signed`] to get an `Option` instead."] impl SubAssign < TimeDelta > for NaiveDate { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { * self = self . sub (rhs) ; } }
    };
}

impl_348!();