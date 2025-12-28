macro_rules! deps {
    () => {
        ExtendedFloat!();
        ExtendedFloatArray!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        # [doc = " Allow indexing of values without bounds checking"] impl ExtendedFloatArray { # [inline] pub fn get_extended_float (& self , index : usize) -> ExtendedFloat { let mant = self . mant [index] ; let exp = self . exp [index] ; ExtendedFloat { mant , exp } } # [inline] pub fn len (& self) -> usize { self . mant . len () } }
    };
}

impl_393!()