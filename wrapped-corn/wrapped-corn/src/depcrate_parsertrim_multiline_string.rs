// Generated macro for trim_multiline_string (function)
macro_rules! Depcrate_parsertrim_multiline_string {
() => {
// Module: crate::parser
// Provides: {"trim_multiline_string"}
// Dependencies: {}
# [doc = " Takes a multiline string and trims the maximum amount of"] # [doc = " whitespace at the start of each line"] # [doc = " while preserving formatting."] # [doc = ""] # [doc = " Based on code from `indoc` crate:"] # [doc = " <https://github.com/dtolnay/indoc/blob/60b5fa29ba4f98b479713621a1f4ec96155caaba/src/unindent.rs#L15-L51>"] fn trim_multiline_string (string : & str) -> String { let ignore_first_line = string . starts_with ('\n') || string . starts_with ("\r\n") ; let spaces = string . lines () . skip (1) . map (| line | line . chars () . take_while (char :: is_ascii_whitespace) . count ()) . min () . unwrap_or_default () ; let mut result = String :: with_capacity (string . len ()) ; for (i , line) in string . lines () . enumerate () { if i > 1 || (i == 1 && ! ignore_first_line) { result . push ('\n') ; } if i == 0 { result . push_str (line) ; } else if line . len () > spaces { result . push_str (& line [spaces ..]) ; } } result }
};
}
