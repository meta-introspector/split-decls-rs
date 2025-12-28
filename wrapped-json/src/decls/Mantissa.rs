macro_rules! deps {
    () => {
        Type!();
        Integer!();
    };
}

macro_rules! Mantissa {
    () => {
        deps!();
        # [doc = " Type trait for the mantissa type."] pub trait Mantissa : Integer { # [doc = " Mask to extract the high bits from the integer."] const HIMASK : Self ; # [doc = " Mask to extract the low bits from the integer."] const LOMASK : Self ; # [doc = " Full size of the integer, in bits."] const FULL : i32 ; # [doc = " Half size of the integer, in bits."] const HALF : i32 = Self :: FULL / 2 ; }
    };
}

Mantissa!()