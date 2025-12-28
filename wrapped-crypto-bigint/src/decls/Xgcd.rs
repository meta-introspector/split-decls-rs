macro_rules! Xgcd {
    () => {
        # [doc = " Compute the extended greatest common divisor of two integers."] pub trait Xgcd < Rhs = Self > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute the extended greatest common divisor of `self` and `rhs`."] fn xgcd (& self , rhs : & Rhs) -> Self :: Output ; # [doc = " Compute the extended greatest common divisor of `self` and `rhs` in variable time."] fn xgcd_vartime (& self , rhs : & Rhs) -> Self :: Output ; }
    };
}

Xgcd!()