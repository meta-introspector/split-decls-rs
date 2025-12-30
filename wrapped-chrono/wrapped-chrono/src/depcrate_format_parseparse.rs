// Generated macro for parse (function)
macro_rules! Depcrate_format_parseparse {
() => {
// Module: crate::format::parse
// Provides: {"parse"}
// Dependencies: {}
# [doc = " Tries to parse given string into `parsed` with given formatting items."] # [doc = " Returns `Ok` when the entire string has been parsed (otherwise `parsed` should not be used)."] # [doc = " There should be no trailing string after parsing;"] # [doc = " use a stray [`Item::Space`](./enum.Item.html#variant.Space) to trim whitespaces."] # [doc = ""] # [doc = " This particular date and time parser is:"] # [doc = ""] # [doc = " - Greedy. It will consume the longest possible prefix."] # [doc = "   For example, `April` is always consumed entirely when the long month name is requested;"] # [doc = "   it equally accepts `Apr`, but prefers the longer prefix in this case."] # [doc = ""] # [doc = " - Padding-agnostic (for numeric items)."] # [doc = "   The [`Pad`](./enum.Pad.html) field is completely ignored,"] # [doc = "   so one can prepend any number of whitespace then any number of zeroes before numbers."] # [doc = ""] # [doc = " - (Still) obeying the intrinsic parsing width. This allows, for example, parsing `HHMMSS`."] pub fn parse < 'a , I , B > (parsed : & mut Parsed , s : & str , items : I) -> ParseResult < () > where I : Iterator < Item = B > , B : Borrow < Item < 'a > > , { match parse_internal (parsed , s , items) { Ok ("") => Ok (()) , Ok (_) => Err (TOO_LONG) , Err (e) => Err (e) , } }
};
}
