// Generated macro for impl_566 (impl)
macro_rules! Depcrate_naive_datetimeimpl_566 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_566"}
// Dependencies: {}
# [doc = " Add `std::time::Duration` to `NaiveDateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_add_signed`] to get an `Option` instead."] impl Add < Duration > for NaiveDateTime { type Output = NaiveDateTime ; # [inline] fn add (self , rhs : Duration) -> NaiveDateTime { let rhs = TimeDelta :: from_std (rhs) . expect ("overflow converting from core::time::Duration to TimeDelta") ; self . checked_add_signed (rhs) . expect ("`NaiveDateTime + TimeDelta` overflowed") } }
};
}
