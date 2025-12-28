macro_rules! ExtendedFloat {
    () => {
        # [doc = " Extended precision floating-point type."] # [doc = ""] # [doc = " Private implementation, exposed only for testing purposes."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct ExtendedFloat { # [doc = " Mantissa for the extended-precision float."] pub mant : u64 , # [doc = " Binary exponent for the extended-precision float."] pub exp : i32 , }
    };
}

ExtendedFloat!();