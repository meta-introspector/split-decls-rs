macro_rules! deps {
    () => {
        IntegerBase!();
        ParseError!();
        Buffer!();
        IntegerLit!();
        FromIntegerLiteral!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < B : Buffer > IntegerLit < B > { # [doc = " Parses the input as an integer literal. Returns an error if the input is"] # [doc = " invalid or represents a different kind of literal."] pub fn parse (input : B) -> Result < Self , ParseError > { match first_byte_or_empty (& input) ? { digit @ b'0' ..= b'9' => { let IntegerLit { start_main_part , end_main_part , base , .. } = parse_impl (& input , digit) ? ; Ok (Self { raw : input , start_main_part , end_main_part , base }) } _ => Err (perr (0 , DoesNotStartWithDigit)) , } } # [doc = " Performs the actual string to int conversion to obtain the integer"] # [doc = " value. The optional type suffix of the literal **is ignored by this"] # [doc = " method**. This means `N` does not need to match the type suffix!"] # [doc = ""] # [doc = " Returns `None` if the literal overflows `N`."] # [doc = ""] # [doc = " Hint: `u128` can represent all possible values integer literal values,"] # [doc = " as there are no negative literals (see type docs). Thus you can, for"] # [doc = " example, safely use `lit.value::<u128>().to_string()` to get a decimal"] # [doc = " string. (Technically, Rust integer literals can represent arbitrarily"] # [doc = " large numbers, but those would be rejected at a later stage by the Rust"] # [doc = " compiler)."] pub fn value < N : FromIntegerLiteral > (& self) -> Option < N > { let base = N :: from_small_number (self . base . value ()) ; let mut acc = N :: from_small_number (0) ; for digit in self . raw_main_part () . bytes () { if digit == b'_' { continue ; } let digit = hex_digit_value (digit) . unwrap_or_else (| | unreachable ! ("bug: integer main part contains non-digit")) ; acc = acc . checked_mul (base) ? ; acc = acc . checked_add (N :: from_small_number (digit)) ? ; } Some (acc) } # [doc = " The base of this integer literal."] pub fn base (& self) -> IntegerBase { self . base } # [doc = " The main part containing the digits and potentially `_`. Do not try to"] # [doc = " parse this directly as that would ignore the base!"] pub fn raw_main_part (& self) -> & str { & (* self . raw) [self . start_main_part .. self . end_main_part] } # [doc = " The optional suffix. Returns `\"\"` if the suffix is empty/does not exist."] # [doc = ""] # [doc = " If you want the type, try `IntegerType::from_suffix(lit.suffix())`."] pub fn suffix (& self) -> & str { & (* self . raw) [self . end_main_part ..] } # [doc = " Returns the raw input that was passed to `parse`."] pub fn raw_input (& self) -> & str { & self . raw } # [doc = " Returns the raw input that was passed to `parse`, potentially owned."] pub fn into_raw_input (self) -> B { self . raw } }
    };
}

impl_187!()