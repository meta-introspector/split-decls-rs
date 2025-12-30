// Generated macro for FormatSpec (struct)
macro_rules! DepcrateFormatSpec {
() => {
// Module: crate
// Provides: {"FormatSpec"}
// Dependencies: {}
# [doc = " Specification for the formatting of an argument in the format string."] # [derive (Copy , Clone , Debug , PartialEq)] pub struct FormatSpec < 'a > { # [doc = " Optionally specified character to fill alignment with."] pub fill : Option < char > , # [doc = " Span of the optionally specified fill character."] pub fill_span : Option < InnerSpan > , # [doc = " Optionally specified alignment."] pub align : Alignment , # [doc = " The `+` or `-` flag."] pub sign : Option < Sign > , # [doc = " The `#` flag."] pub alternate : bool , # [doc = " The `0` flag."] pub zero_pad : bool , # [doc = " The `x` or `X` flag. (Only for `Debug`.)"] pub debug_hex : Option < DebugHex > , # [doc = " The integer precision to use."] pub precision : Count < 'a > , # [doc = " The span of the precision formatting flag (for diagnostics)."] pub precision_span : Option < InnerSpan > , # [doc = " The string width requested for the resulting format."] pub width : Count < 'a > , # [doc = " The span of the width formatting flag (for diagnostics)."] pub width_span : Option < InnerSpan > , # [doc = " The descriptor string representing the name of the format desired for"] # [doc = " this argument, this can be empty or any number of characters, although"] # [doc = " it is required to be one word."] pub ty : & 'a str , # [doc = " The span of the descriptor string (for diagnostics)."] pub ty_span : Option < InnerSpan > , }
};
}
