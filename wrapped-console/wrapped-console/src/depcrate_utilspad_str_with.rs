// Generated macro for pad_str_with (function)
macro_rules! Depcrate_utilspad_str_with {
() => {
// Module: crate::utils
// Provides: {"pad_str_with"}
// Dependencies: {}
# [doc = " Pads a string with specific padding to fill a certain number of characters."] # [doc = ""] # [doc = " This will honor ansi codes correctly and allows you to align a string"] # [doc = " on the left, right or centered.  Additionally truncation can be enabled"] # [doc = " by setting `truncate` to a string that should be used as a truncation"] # [doc = " marker."] pub fn pad_str_with < 'a > (s : & 'a str , width : usize , align : Alignment , truncate : Option < & str > , pad : char ,) -> Cow < 'a , str > { let cols = measure_text_width (s) ; if cols >= width { return match truncate { None => Cow :: Borrowed (s) , Some (tail) => truncate_str (s , width , tail) , } ; } let diff = width - cols ; let (left_pad , right_pad) = match align { Alignment :: Left => (0 , diff) , Alignment :: Right => (diff , 0) , Alignment :: Center => (diff / 2 , diff - diff / 2) , } ; let mut rv = String :: new () ; for _ in 0 .. left_pad { rv . push (pad) ; } rv . push_str (s) ; for _ in 0 .. right_pad { rv . push (pad) ; } Cow :: Owned (rv) }
};
}
