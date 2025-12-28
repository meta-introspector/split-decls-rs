macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveTime!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        # [doc = " Subtract-assign `TimeDelta` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl SubAssign < TimeDelta > for NaiveTime { # [inline] fn sub_assign (& mut self , rhs : TimeDelta) { * self = self . sub (rhs) ; } }
    };
}

impl_514!();