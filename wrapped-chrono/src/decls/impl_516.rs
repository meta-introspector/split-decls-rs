macro_rules! deps {
    () => {
        NaiveTime!();
        Duration!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        # [doc = " Subtract-assign `std::time::Duration` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl SubAssign < Duration > for NaiveTime { # [inline] fn sub_assign (& mut self , rhs : Duration) { * self = * self - rhs ; } }
    };
}

impl_516!();