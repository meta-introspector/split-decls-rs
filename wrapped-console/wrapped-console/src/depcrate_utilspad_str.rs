// Generated macro for pad_str (function)
macro_rules! Depcrate_utilspad_str {
() => {
// Module: crate::utils
// Provides: {"pad_str"}
// Dependencies: {}
# [doc = " Pads a string to fill a certain number of characters."] # [doc = ""] # [doc = " This will honor ansi codes correctly and allows you to align a string"] # [doc = " on the left, right or centered.  Additionally truncation can be enabled"] # [doc = " by setting `truncate` to a string that should be used as a truncation"] # [doc = " marker."] pub fn pad_str < 'a > (s : & 'a str , width : usize , align : Alignment , truncate : Option < & str > ,) -> Cow < 'a , str > { pad_str_with (s , width , align , truncate , ' ') }
};
}
