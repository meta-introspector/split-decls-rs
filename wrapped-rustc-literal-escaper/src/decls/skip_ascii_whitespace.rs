macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! skip_ascii_whitespace {
    () => {
        deps!();
        # [doc = " Interpret a string continuation escape (https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-continuation-escapes)"] # [doc = ""] # [doc = " Skip ASCII whitespace, except for the formfeed character"] # [doc = " (see [this issue](https://github.com/rust-lang/rust/issues/136600))."] # [doc = " Warns on unescaped newline and following non-ASCII whitespace."] # [inline] fn skip_ascii_whitespace (chars : & mut Chars < '_ > , start : usize , mut callback : impl FnMut (Range < usize > , EscapeError) ,) { let rest = chars . as_str () ; let first_non_space = rest . bytes () . position (| b | b != b' ' && b != b'\t' && b != b'\n' && b != b'\r') . unwrap_or (rest . len ()) ; let (space , rest) = rest . split_at (first_non_space) ; let end = start + 2 + first_non_space ; if space . contains ('\n') { callback (start .. end , EscapeError :: MultipleSkippedLinesWarning) ; } * chars = rest . chars () ; if let Some (c) = chars . clone () . next () { if c . is_whitespace () { callback (start .. end + c . len_utf8 () , EscapeError :: UnskippedWhitespaceWarning ,) ; } } }
    };
}

skip_ascii_whitespace!();