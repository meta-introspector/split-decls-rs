macro_rules! deps {
    () => {
        Duration!();
        NaiveDateTime!();
        TimeDelta!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        # [doc = " Subtract `std::time::Duration` from `NaiveDateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling] the subtraction assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_sub_signed`] to get an `Option` instead."] impl Sub < Duration > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn sub (self , rhs : Duration) -> NaiveDateTime { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; self . checked_sub_signed (rhs) . expect ("`NaiveDateTime - TimeDelta` overflowed") } }
    };
}

impl_426!()