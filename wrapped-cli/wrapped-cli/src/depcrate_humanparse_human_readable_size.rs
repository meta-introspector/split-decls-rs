// Generated macro for parse_human_readable_size (function)
macro_rules! Depcrate_humanparse_human_readable_size {
() => {
// Module: crate::human
// Provides: {"parse_human_readable_size"}
// Dependencies: {}
# [doc = " Parse a human readable size like `2M` into a corresponding number of bytes."] # [doc = ""] # [doc = " Supported size suffixes are `K` (for kilobyte), `M` (for megabyte) and `G`"] # [doc = " (for gigabyte). If a size suffix is missing, then the size is interpreted"] # [doc = " as bytes. If the size is too big to fit into a `u64`, then this returns an"] # [doc = " error."] # [doc = ""] # [doc = " Additional suffixes may be added over time."] pub fn parse_human_readable_size (size : & str) -> Result < u64 , ParseSizeError > { let digits_end = size . as_bytes () . iter () . take_while (| & b | b . is_ascii_digit ()) . count () ; let digits = & size [.. digits_end] ; if digits . is_empty () { return Err (ParseSizeError :: format (size)) ; } let value = digits . parse :: < u64 > () . map_err (| e | ParseSizeError :: int (size , e)) ? ; let suffix = & size [digits_end ..] ; if suffix . is_empty () { return Ok (value) ; } let bytes = match suffix { "K" => value . checked_mul (1 << 10) , "M" => value . checked_mul (1 << 20) , "G" => value . checked_mul (1 << 30) , _ => return Err (ParseSizeError :: format (size)) , } ; bytes . ok_or_else (| | ParseSizeError :: overflow (size)) }
};
}
