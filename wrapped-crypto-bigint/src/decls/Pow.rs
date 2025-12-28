macro_rules! Pow {
    () => {
        # [doc = " Constant-time exponentiation."] pub trait Pow < Exponent > { # [doc = " Raises to the `exponent` power."] fn pow (& self , exponent : & Exponent) -> Self ; }
    };
}

Pow!();