// Generated macro for deunicode_with_tofu (function)
macro_rules! Depcratedeunicode_with_tofu {
() => {
// Module: crate
// Provides: {"deunicode_with_tofu"}
// Dependencies: {}
# [doc = " Same as [`deunicode()`], but unknown characters can be replaced with a custom string."] # [doc = ""] # [doc = " You can use \"\\u{FFFD}\" to use the usual Unicode Replacement Character."] # [doc = ""] # [doc = " \"Tofu\" is a nickname for a replacement character, which in Unicode fonts usually"] # [doc = " looks like a block of tofu."] # [inline] # [cfg (feature = "alloc")] # [must_use] pub fn deunicode_with_tofu (s : & str , custom_placeholder : & str) -> String { deunicode_with_tofu_cow (s , custom_placeholder) . into_owned () }
};
}
