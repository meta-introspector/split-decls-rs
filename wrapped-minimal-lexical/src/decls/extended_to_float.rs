macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! extended_to_float {
    () => {
        deps!();
        # [doc = " Converts an `ExtendedFloat` to the closest machine float type."] # [inline (always)] pub fn extended_to_float < F : Float > (x : ExtendedFloat) -> F { let mut word = x . mant ; word |= (x . exp as u64) << F :: MANTISSA_SIZE ; F :: from_bits (word) }
    };
}

extended_to_float!()