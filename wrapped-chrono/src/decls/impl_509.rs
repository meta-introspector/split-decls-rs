macro_rules! deps {
    () => {
        NaiveTime!();
        TimeDelta!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        # [doc = " Add-assign `TimeDelta` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl AddAssign < TimeDelta > for NaiveTime { # [inline] fn add_assign (& mut self , rhs : TimeDelta) { * self = self . add (rhs) ; } }
    };
}

impl_509!();