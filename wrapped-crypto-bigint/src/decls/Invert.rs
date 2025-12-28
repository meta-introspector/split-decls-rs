macro_rules! Invert {
    () => {
        # [doc = " Constant-time inversion."] pub trait Invert { # [doc = " Output of the inversion."] type Output ; # [doc = " Computes the inverse."] fn invert (& self) -> Self :: Output ; # [doc = " Computes the inverse in variable-time."] fn invert_vartime (& self) -> Self :: Output { self . invert () } }
    };
}

Invert!()