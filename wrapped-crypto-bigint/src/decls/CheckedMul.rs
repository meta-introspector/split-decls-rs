macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! CheckedMul {
    () => {
        deps!();
        # [doc = " Checked multiplication."] pub trait CheckedMul < Rhs = Self > : Sized { # [doc = " Perform checked multiplication, returning a [`CtOption`] which `is_some`"] # [doc = " only if the operation did not overflow."] fn checked_mul (& self , rhs : & Rhs) -> CtOption < Self > ; }
    };
}

CheckedMul!();