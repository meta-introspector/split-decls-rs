// Generated macro for take_until_unbalanced (function)
macro_rules! Depcrate_log_format_parsertake_until_unbalanced {
() => {
// Module: crate::log::format::parser
// Provides: {"take_until_unbalanced"}
// Dependencies: {}
# [doc = " This function is taken as-is from the parse-hyperlinks crate"] # [doc = " https://docs.rs/parse-hyperlinks/0.9.3/src/parse_hyperlinks/lib.rs.html#24-68"] # [doc = " There is an open issue in nom to include this parser in nom 8.0"] # [doc = " https://github.com/rust-bakery/nom/issues/1253"] pub fn take_until_unbalanced (opening_bracket : char , closing_bracket : char ,) -> impl Fn (& str) -> IResult < & str , & str , () > { move | i : & str | { let mut index = 0 ; let mut bracket_counter = 0 ; while let Some (n) = & i [index ..] . find (& [opening_bracket , closing_bracket , '\\'] [..]) { index += n ; let mut it = i [index ..] . chars () ; match it . next () . unwrap_or_default () { '\\' => { index += '\\' . len_utf8 () ; let c = it . next () . unwrap_or_default () ; index += c . len_utf8 () ; } c if c == opening_bracket => { bracket_counter += 1 ; index += opening_bracket . len_utf8 () ; } c if c == closing_bracket => { bracket_counter -= 1 ; index += closing_bracket . len_utf8 () ; } _ => unreachable ! () , } ; if bracket_counter == - 1 { index -= closing_bracket . len_utf8 () ; return Ok ((& i [index ..] , & i [0 .. index])) ; } ; } if bracket_counter == 0 { Ok (("" , i)) } else { Err (nom :: Err :: Error (())) } } }
};
}
