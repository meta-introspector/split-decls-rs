macro_rules! deps {
    () => {
        NaiveDateTime!();
        FixedOffset!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        # [doc = " Add `FixedOffset` to `NaiveDateTime`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_add_offset` to get an `Option` instead."] impl Add < FixedOffset > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn add (self , rhs : FixedOffset) -> NaiveDateTime { self . checked_add_offset (rhs) . expect ("`NaiveDateTime + FixedOffset` out of range") } }
    };
}

impl_423!()