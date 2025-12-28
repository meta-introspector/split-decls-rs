macro_rules! deps {
    () => {
        Item!();
        Parsed!();
        ParseResult!();
    };
}

macro_rules! parse_and_remainder {
    () => {
        deps!();
        # [doc = " Tries to parse given string into `parsed` with given formatting items."] # [doc = " Returns `Ok` with a slice of the unparsed remainder."] # [doc = ""] # [doc = " This particular date and time parser is:"] # [doc = ""] # [doc = " - Greedy. It will consume the longest possible prefix."] # [doc = "   For example, `April` is always consumed entirely when the long month name is requested;"] # [doc = "   it equally accepts `Apr`, but prefers the longer prefix in this case."] # [doc = ""] # [doc = " - Padding-agnostic (for numeric items)."] # [doc = "   The [`Pad`](./enum.Pad.html) field is completely ignored,"] # [doc = "   so one can prepend any number of zeroes before numbers."] # [doc = ""] # [doc = " - (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`."] pub fn parse_and_remainder < 'a , 'b , I , B > (parsed : & mut Parsed , s : & 'b str , items : I ,) -> ParseResult < & 'b str > where I : Iterator < Item = B > , B : Borrow < Item < 'a > > , { parse_internal (parsed , s , items) }
    };
}

parse_and_remainder!();