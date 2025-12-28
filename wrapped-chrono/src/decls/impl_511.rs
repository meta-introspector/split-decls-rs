macro_rules! deps {
    () => {
        NaiveTime!();
        Duration!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        # [doc = " Add-assign `std::time::Duration` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl AddAssign < Duration > for NaiveTime { # [inline] fn add_assign (& mut self , rhs : Duration) { * self = * self + rhs ; } }
    };
}

impl_511!();