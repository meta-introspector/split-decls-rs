macro_rules! deps {
    () => {
        NaiveDateTime!();
        Duration!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        # [doc = " Subtract-assign `std::time::Duration` from `NaiveDateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_sub_signed`] to get an `Option` instead."] impl SubAssign < Duration > for NaiveDateTime { # [inline] fn sub_assign (& mut self , rhs : Duration) { * self = self . sub (rhs) ; } }
    };
}

impl_428!();