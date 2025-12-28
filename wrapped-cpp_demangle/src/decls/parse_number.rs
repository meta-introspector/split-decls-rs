macro_rules! deps {
    () => {
        Parse!();
        Result!();
        Error!();
        IndexStr!();
    };
}

macro_rules! parse_number {
    () => {
        deps!();
        # [doc = " Parse a number with the given `base`. Do not allow negative numbers"] # [doc = " (prefixed with an 'n' instead of a '-') if `allow_signed` is false."] # [allow (unsafe_code)] fn parse_number (base : u32 , allow_signed : bool , mut input : IndexStr) -> Result < (isize , IndexStr) > { if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } let num_is_negative = if allow_signed && input . as_ref () [0] == b'n' { input = input . range_from (1 ..) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } true } else { false } ; let num_numeric = input . as_ref () . iter () . map (| & c | c as char) . take_while (| c | c . is_digit (base) && (c . is_numeric () || c . is_uppercase ())) . count () ; if num_numeric == 0 { return Err (error :: Error :: UnexpectedText) ; } let (head , tail) = input . split_at (num_numeric) ; let head = head . as_ref () ; if num_numeric > 1 && head [0] == b'0' { return Err (error :: Error :: UnexpectedText) ; } let head = unsafe { str :: from_utf8_unchecked (head) } ; let mut number = isize :: from_str_radix (head , base) . map_err (| _ | error :: Error :: Overflow) ? ; if num_is_negative { number = - number ; } Ok ((number , tail)) }
    };
}

parse_number!()