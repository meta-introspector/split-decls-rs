// Generated macro for short_month_padding (function)
macro_rules! Depcrate_output_timeshort_month_padding {
() => {
// Module: crate::output::time
// Provides: {"short_month_padding"}
// Dependencies: {}
# [doc = " Convert between Unicode width and width in chars to use in format!."] # [doc = " ex: in Japanese, 月 is one character, but it has the width of two."] # [doc = " For alignment purposes, we take the real display width into account."] # [doc = " So, `MAXIMUM_MONTH_WIDTH` (“12月”) = 4, but if we use `{:4}` in format!,"] # [doc = " it will add a space (“ 12月”) because format! counts characters."] # [doc = " Conversely, a char can have a width of zero (like combining diacritics)"] fn short_month_padding (max_month_width : usize , month : & str) -> usize { let shift = month . chars () . count () as isize - UnicodeWidthStr :: width (month) as isize ; (max_month_width as isize + shift) as usize }
};
}
