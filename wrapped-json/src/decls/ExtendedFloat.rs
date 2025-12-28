macro_rules! deps {
    () => {
        Mantissa!();
    };
}

macro_rules! ExtendedFloat {
    () => {
        deps!();
        # [doc = " Extended precision floating-point type."] # [doc = ""] # [doc = " Private implementation, exposed only for testing purposes."] # [doc (hidden)] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub (crate) struct ExtendedFloat { # [doc = " Mantissa for the extended-precision float."] pub mant : u64 , # [doc = " Binary exponent for the extended-precision float."] pub exp : i32 , }
    };
}

ExtendedFloat!();