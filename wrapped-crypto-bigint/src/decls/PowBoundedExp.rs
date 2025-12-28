macro_rules! PowBoundedExp {
    () => {
        # [doc = " Constant-time exponentiation with exponent of a bounded bit size."] pub trait PowBoundedExp < Exponent > { # [doc = " Raises to the `exponent` power,"] # [doc = " with `exponent_bits` representing the number of (least significant) bits"] # [doc = " to take into account for the exponent."] # [doc = ""] # [doc = " NOTE: `exponent_bits` may be leaked in the time pattern."] fn pow_bounded_exp (& self , exponent : & Exponent , exponent_bits : u32) -> Self ; }
    };
}

PowBoundedExp!()