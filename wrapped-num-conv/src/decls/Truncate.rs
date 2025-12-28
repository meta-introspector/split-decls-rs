macro_rules! deps {
    () => {
        TruncateTarget!();
    };
}

macro_rules! Truncate {
    () => {
        deps!();
        # [doc = " Truncate to an integer of the same size or smaller, preserving the least significant bits."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use num_conv::Truncate;"] # [doc = " assert_eq!(u16::MAX.truncate::<u8>(), u8::MAX);"] # [doc = " assert_eq!(u32::MAX.truncate::<u16>(), u16::MAX);"] # [doc = " assert_eq!(u64::MAX.truncate::<u32>(), u32::MAX);"] # [doc = " assert_eq!(u128::MAX.truncate::<u64>(), u64::MAX);"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use num_conv::Truncate;"] # [doc = " assert_eq!((-1_i16).truncate::<i8>(), -1_i8);"] # [doc = " assert_eq!((-1_i32).truncate::<i16>(), -1_i16);"] # [doc = " assert_eq!((-1_i64).truncate::<i32>(), -1_i32);"] # [doc = " assert_eq!((-1_i128).truncate::<i64>(), -1_i64);"] # [doc = " ```"] pub trait Truncate : sealed :: Integer { # [doc = " Truncate an integer to an integer of the same size or smaller, preserving the least"] # [doc = " significant bits."] fn truncate < T > (self) -> T where Self : TruncateTarget < T > ; }
    };
}

Truncate!();