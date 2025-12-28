macro_rules! SquareRoot {
    () => {
        # [doc = " Support for calucaling square roots."] pub trait SquareRoot { # [doc = " Computes `floor(sqrt(self))`."] fn sqrt (& self) -> Self ; # [doc = " Computes `floor(sqrt(self))`, variable time in `self`."] fn sqrt_vartime (& self) -> Self ; }
    };
}

SquareRoot!();