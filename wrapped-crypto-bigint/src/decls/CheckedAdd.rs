macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! CheckedAdd {
    () => {
        deps!();
        # [doc = " Checked addition."] pub trait CheckedAdd < Rhs = Self > : Sized { # [doc = " Perform checked addition, returning a [`CtOption`] which `is_some` only if the operation"] # [doc = " did not overflow."] fn checked_add (& self , rhs : & Rhs) -> CtOption < Self > ; }
    };
}

CheckedAdd!();