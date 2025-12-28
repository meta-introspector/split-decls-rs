macro_rules! deps {
    () => {
        Duration!();
        NaiveDateTime!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        # [doc = " Add-assign `std::time::Duration` to `NaiveDateTime`."] # [doc = ""] # [doc = " As a part of Chrono's [leap second handling], the addition assumes that **there is no leap"] # [doc = " second ever**, except when the `NaiveDateTime` itself represents a leap  second in which case"] # [doc = " the assumption becomes that **there is exactly a single leap second ever**."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_add_signed`] to get an `Option` instead."] impl AddAssign < Duration > for NaiveDateTime { # [inline] fn add_assign (& mut self , rhs : Duration) { * self = self . add (rhs) ; } }
    };
}

impl_422!();