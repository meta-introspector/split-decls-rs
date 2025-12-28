macro_rules! deps {
    () => {
        FloatLit!();
        ParseError!();
        Buffer!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < B : Buffer > FloatLit < B > { # [doc = " Parses the input as a floating point literal. Returns an error if the"] # [doc = " input is invalid or represents a different kind of literal. Will also"] # [doc = " reject decimal integer literals like `23` or `17f32`, in accordance"] # [doc = " with the spec."] pub fn parse (s : B) -> Result < Self , ParseError > { match first_byte_or_empty (& s) ? { b'0' ..= b'9' => { let FloatLit { end_integer_part , end_fractional_part , end_number_part , .. } = parse_impl (& s) ? ; Ok (Self { raw : s , end_integer_part , end_fractional_part , end_number_part }) } _ => Err (perr (0 , DoesNotStartWithDigit)) , } } # [doc = " Returns the number part (including integer part, fractional part and"] # [doc = " exponent), but without the suffix. If you want an actual floating"] # [doc = " point value, you need to parse this string, e.g. with `f32::from_str`"] # [doc = " or an external crate."] pub fn number_part (& self) -> & str { & (* self . raw) [.. self . end_number_part] } # [doc = " Returns the non-empty integer part of this literal."] pub fn integer_part (& self) -> & str { & (* self . raw) [.. self . end_integer_part] } # [doc = " Returns the optional fractional part of this literal. Does not include"] # [doc = " the period. If a period exists in the input, `Some` is returned, `None`"] # [doc = " otherwise. Note that `Some(\"\")` might be returned, e.g. for `3.`."] pub fn fractional_part (& self) -> Option < & str > { if self . end_integer_part == self . end_fractional_part { None } else { Some (& (* self . raw) [self . end_integer_part + 1 .. self . end_fractional_part]) } } # [doc = " Optional exponent part. Might be empty if there was no exponent part in"] # [doc = " the input. Includes the `e` or `E` at the beginning."] pub fn exponent_part (& self) -> & str { & (* self . raw) [self . end_fractional_part .. self . end_number_part] } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] pub fn suffix (& self) -> & str { & (* self . raw) [self . end_number_part ..] } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } }
    };
}

impl_131!();