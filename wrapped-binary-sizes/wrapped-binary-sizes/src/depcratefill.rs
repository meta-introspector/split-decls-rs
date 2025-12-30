// Generated macro for fill (function)
macro_rules! Depcratefill {
() => {
// Module: crate
// Provides: {"fill"}
// Dependencies: {}
# [cfg (not (feature = "textwrap"))] # [doc = " Quick-and-dirty fill implementation."] # [doc = ""] # [doc = " Assumes single space between words, assumes 1 column per Unicode"] # [doc = " character (no emoji handling) and assumes that the longest word"] # [doc = " fit on the line (no handling of hyphens or over-long words)."] fn fill (text : & str , width : usize) -> String { let mut result = String :: with_capacity (text . len ()) ; let mut line_width = 0 ; for word in text . split_whitespace () { if line_width + 1 + word . len () > width { result . push ('\n') ; line_width = 0 ; } result . push_str (word) ; result . push (' ') ; line_width += word . len () + 1 ; } result . truncate (result . len () - 1) ; result }
};
}
