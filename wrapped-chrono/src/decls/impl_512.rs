macro_rules! deps {
    () => {
        FixedOffset!();
        NaiveTime!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        # [doc = " Add `FixedOffset` to `NaiveTime`."] # [doc = ""] # [doc = " This wraps around and never overflows or underflows."] # [doc = " In particular the addition ignores integral number of days."] impl Add < FixedOffset > for NaiveTime { type Output = NaiveTime ; # [inline] fn add (self , rhs : FixedOffset) -> NaiveTime { self . overflowing_add_offset (rhs) . 0 } }
    };
}

impl_512!()