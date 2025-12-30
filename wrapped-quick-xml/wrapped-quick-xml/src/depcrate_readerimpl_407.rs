// Generated macro for impl_407 (impl)
macro_rules! Depcrate_readerimpl_407 {
() => {
// Module: crate::reader
// Provides: {"impl_407"}
// Dependencies: {}
impl Config { # [doc = " Set both [`trim_text_start`] and [`trim_text_end`] to the same value."] # [doc = ""] # [doc = " <div style=\"background:rgba(80, 240, 100, 0.20);padding:0.75em;\">"] # [doc = ""] # [doc = " WARNING: With this option every text events will be trimmed which is"] # [doc = " incorrect behavior when text events delimited by comments, processing"] # [doc = " instructions or CDATA sections. To correctly trim data manually apply"] # [doc = " [`BytesText::inplace_trim_start`] and [`BytesText::inplace_trim_end`]"] # [doc = " only to necessary events."] # [doc = " </div>"] # [doc = ""] # [doc = " [`trim_text_start`]: Self::trim_text_start"] # [doc = " [`trim_text_end`]: Self::trim_text_end"] # [doc = " [`BytesText::inplace_trim_start`]: crate::events::BytesText::inplace_trim_start"] # [doc = " [`BytesText::inplace_trim_end`]: crate::events::BytesText::inplace_trim_end"] # [inline] pub fn trim_text (& mut self , trim : bool) { self . trim_text_start = trim ; self . trim_text_end = trim ; } # [doc = " Turn on or off all checks for well-formedness. Currently it is that settings:"] # [doc = " - [`check_comments`](Self::check_comments)"] # [doc = " - [`check_end_names`](Self::check_end_names)"] # [inline] pub fn enable_all_checks (& mut self , enable : bool) { self . check_comments = enable ; self . check_end_names = enable ; } }
};
}
