macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! parse_float {
    () => {
        deps!();
        # [doc = " Parse float from extracted float components."] # [doc = ""] # [doc = " * `integer`     - Cloneable, forward iterator over integer digits."] # [doc = " * `fraction`    - Cloneable, forward iterator over integer digits."] # [doc = " * `exponent`    - Parsed, 32-bit exponent."] # [doc = ""] # [doc = " # Preconditions"] # [doc = " 1. The integer should not have leading zeros."] # [doc = " 2. The fraction should not have trailing zeros."] # [doc = " 3. All bytes in `integer` and `fraction` should be valid digits,"] # [doc = "     in the range [`b'0', b'9']."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Although passing garbage input will not cause memory safety issues,"] # [doc = " it is very likely to cause a panic with a large number of digits, or"] # [doc = " in debug mode. The big-integer arithmetic without the `alloc` feature"] # [doc = " assumes a maximum, fixed-width input, which assumes at maximum a"] # [doc = " value of `10^(769 + 342)`, or ~4000 bits of storage. Passing in"] # [doc = " nonsensical digits may require up to ~6000 bits of storage, which will"] # [doc = " panic when attempting to add it to the big integer. It is therefore"] # [doc = " up to the caller to validate this input."] # [doc = ""] # [doc = " We cannot efficiently remove trailing zeros while only accepting a"] # [doc = " forward iterator."] pub fn parse_float < 'a , F , Iter1 , Iter2 > (integer : Iter1 , fraction : Iter2 , exponent : i32) -> F where F : Float , Iter1 : Iterator < Item = & 'a u8 > + Clone , Iter2 : Iterator < Item = & 'a u8 > + Clone , { let num = parse_number (integer . clone () , fraction . clone () , exponent) ; if let Some (value) = num . try_fast_path () { return value ; } let mut fp = moderate_path :: < F > (& num) ; if fp . exp < 0 { fp . exp -= F :: INVALID_FP ; fp = slow :: < F , _ , _ > (num , fp , integer , fraction) ; } extended_to_float :: < F > (fp) }
    };
}

parse_float!();