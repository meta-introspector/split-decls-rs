macro_rules! nth_bit {
    () => {
        # [doc = " Calculate a scalar factor of 2 above the halfway point."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use minimal_lexical::mask::nth_bit;"] # [doc = " # pub fn main() {"] # [doc = " assert_eq!(nth_bit(2), 0b100);"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn nth_bit (n : u64) -> u64 { debug_assert ! (n < 64 , "nth_bit() overflow in shl.") ; 1 << n }
    };
}

nth_bit!();