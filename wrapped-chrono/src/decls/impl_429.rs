macro_rules! deps {
    () => {
        NaiveDateTime!();
        FixedOffset!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        # [doc = " Subtract `FixedOffset` from `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_sub_offset` to get an `Option` instead."] impl Sub < FixedOffset > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn sub (self , rhs : FixedOffset) -> NaiveDateTime { self . checked_sub_offset (rhs) . expect ("`NaiveDateTime - FixedOffset` out of range") } }
    };
}

impl_429!()