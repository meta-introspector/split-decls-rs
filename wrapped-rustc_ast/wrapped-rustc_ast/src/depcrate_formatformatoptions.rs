// Generated macro for FormatOptions (struct)
macro_rules! Depcrate_formatFormatOptions {
() => {
// Module: crate::format
// Provides: {"FormatOptions"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Default , Debug , PartialEq , Eq)] pub struct FormatOptions { # [doc = " The width. E.g. `{:5}` or `{:width$}`."] pub width : Option < FormatCount > , # [doc = " The precision. E.g. `{:.5}` or `{:.precision$}`."] pub precision : Option < FormatCount > , # [doc = " The alignment. E.g. `{:>}` or `{:<}` or `{:^}`."] pub alignment : Option < FormatAlignment > , # [doc = " The fill character. E.g. the `.` in `{:.>10}`."] pub fill : Option < char > , # [doc = " The `+` or `-` flag."] pub sign : Option < FormatSign > , # [doc = " The `#` flag."] pub alternate : bool , # [doc = " The `0` flag. E.g. the `0` in `{:02x}`."] pub zero_pad : bool , # [doc = " The `x` or `X` flag (for `Debug` only). E.g. the `x` in `{:x?}`."] pub debug_hex : Option < FormatDebugHex > , }
};
}
