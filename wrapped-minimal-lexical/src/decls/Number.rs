macro_rules! Number {
    () => {
        # [doc = " Representation of a number as the significant digits and exponent."] # [doc = ""] # [doc = " This is only used if the exponent base and the significant digit"] # [doc = " radix are the same, since we need to be able to move powers in and"] # [doc = " out of the exponent."] # [derive (Clone , Copy , Debug , Default , PartialEq , Eq)] pub struct Number { # [doc = " The exponent of the float, scaled to the mantissa."] pub exponent : i32 , # [doc = " The significant digits of the float."] pub mantissa : u64 , # [doc = " If the significant digits were truncated."] pub many_digits : bool , }
    };
}

Number!()