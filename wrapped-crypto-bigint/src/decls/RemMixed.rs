macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! RemMixed {
    () => {
        deps!();
        # [doc = " Support for calculating the remainder of two differently sized integers."] pub trait RemMixed < Reductor > : Sized { # [doc = " Calculate the remainder of `self` by the `reductor`."] fn rem_mixed (& self , reductor : & NonZero < Reductor >) -> Reductor ; }
    };
}

RemMixed!();