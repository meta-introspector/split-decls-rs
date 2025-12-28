macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! DivVartime {
    () => {
        deps!();
        # [doc = " Division in variable time."] pub trait DivVartime : Sized { # [doc = " Computes `self / rhs` in variable time."] fn div_vartime (& self , rhs : & NonZero < Self >) -> Self ; }
    };
}

DivVartime!();