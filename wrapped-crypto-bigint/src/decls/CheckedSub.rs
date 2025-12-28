macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! CheckedSub {
    () => {
        deps!();
        # [doc = " Checked subtraction."] pub trait CheckedSub < Rhs = Self > : Sized { # [doc = " Perform checked subtraction, returning a [`CtOption`] which `is_some`"] # [doc = " only if the operation did not underflow."] fn checked_sub (& self , rhs : & Rhs) -> CtOption < Self > ; }
    };
}

CheckedSub!();