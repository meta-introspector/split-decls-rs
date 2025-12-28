macro_rules! deps {
    () => {
        NaiveTime!();
        FixedOffset!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        # [doc = " Subtract `FixedOffset` from `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the subtraction ignores integral number of days."] impl Sub < FixedOffset > for NaiveTime { type Output = NaiveTime ; # [inline] fn sub (self , rhs : FixedOffset) -> NaiveTime { self . overflowing_sub_offset (rhs) . 0 } }
    };
}

impl_517!();