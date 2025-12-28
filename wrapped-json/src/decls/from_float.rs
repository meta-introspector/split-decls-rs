macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! from_float {
    () => {
        deps!();
        # [inline] pub (crate) fn from_float < F > (f : F) -> ExtendedFloat where F : Float , { ExtendedFloat { mant : u64 :: as_cast (f . mantissa ()) , exp : f . exponent () , } }
    };
}

from_float!()