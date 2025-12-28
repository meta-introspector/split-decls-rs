macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! CheckedDiv {
    () => {
        deps!();
        # [doc = " Checked division."] pub trait CheckedDiv < Rhs = Self > : Sized { # [doc = " Perform checked division, returning a [`CtOption`] which `is_some` only if the divisor is"] # [doc = " non-zero."] fn checked_div (& self , rhs : & Rhs) -> CtOption < Self > ; }
    };
}

CheckedDiv!()