macro_rules! deps {
    () => {
        ErrorKind!();
        MinNumTraits!();
        ParseIntegerError!();
    };
}

macro_rules! to_signed_with_radix {
    () => {
        deps!();
        # [doc = " Converts a byte slice in a given base to an integer."] # [doc = ""] # [doc = " Like [`to_unsigned_with_radix`], but numbers may optionally start with a sign"] # [doc = " (`-` or `+`)."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns [`ParseIntegerError`] for any of the following conditions:"] # [doc = ""] # [doc = " * `bytes` has no digits"] # [doc = " * not all characters of `bytes` are `0-9`, `a-z`, `A-Z`, excluding an"] # [doc = "   optional leading sign"] # [doc = " * not all characters refer to digits in the given `radix`, excluding an"] # [doc = "   optional leading sign"] # [doc = " * the number overflows or underflows `I`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `radix` is not in the range `2..=36` (or in the pathological"] # [doc = " case that there is no representation of `radix` in `I`)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use gix_utils::btoi::to_signed_with_radix;"] # [doc = " assert_eq!(Ok(10), to_signed_with_radix(b\"a\", 16));"] # [doc = " assert_eq!(Ok(10), to_signed_with_radix(b\"+a\", 16));"] # [doc = " assert_eq!(Ok(-42), to_signed_with_radix(b\"-101010\", 2));"] # [doc = " ```"] pub fn to_signed_with_radix < I : MinNumTraits > (bytes : & [u8] , radix : u32) -> Result < I , ParseIntegerError > { assert ! ((2 ..= 36) . contains (& radix) , "radix must lie in the range 2..=36, found {radix}") ; let base = I :: from_u32 (radix) . expect ("radix can be represented as integer") ; if bytes . is_empty () { return Err (ParseIntegerError { kind : ErrorKind :: Empty }) ; } let digits = match bytes [0] { b'+' => return to_unsigned_with_radix (& bytes [1 ..] , radix) , b'-' => & bytes [1 ..] , _ => return to_unsigned_with_radix (bytes , radix) , } ; if digits . is_empty () { return Err (ParseIntegerError { kind : ErrorKind :: Empty }) ; } let mut result = I :: ZERO ; for & digit in digits { let x = match char :: from (digit) . to_digit (radix) . and_then (I :: from_u32) { Some (x) => x , None => { return Err (ParseIntegerError { kind : ErrorKind :: InvalidDigit , }) } } ; result = match result . checked_mul (base) { Some (result) => result , None => { return Err (ParseIntegerError { kind : ErrorKind :: Underflow , }) } } ; result = match result . checked_sub (x) { Some (result) => result , None => { return Err (ParseIntegerError { kind : ErrorKind :: Underflow , }) } } ; } Ok (result) }
    };
}

to_signed_with_radix!()