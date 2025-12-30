// Generated macro for impl_333 (impl)
macro_rules! Depcrate_util_span_substringimpl_333 {
() => {
// Module: crate::util::span_substring
// Provides: {"impl_333"}
// Dependencies: {}
impl Span { # [doc = " Given a `Span` and a string, form the resulting string selected exclusively (as in `[start..end`]) by the `Span`"] # [doc = " or [`None`] if the span is out of bounds of the string at either end."] # [must_use] pub fn substring_exclusive (& self , s : & str) -> Option < String > { use alloc :: vec :: Vec ; use unicode_segmentation :: UnicodeSegmentation ; if let (Some (start) , Some (end)) = (self . start . grapheme_index (s) , self . end . grapheme_index (s)) { Some (s . graphemes (true) . collect :: < Vec < & str > > () [start .. end] . concat ()) } else { None } } # [doc = " Given a `Span` and a string, form the resulting string selected inclusively (as in `[start..=end]`) by the `Span`"] # [doc = " or [`None`] if the span is out of bounds of the string at either end."] # [must_use] pub fn substring_inclusive (& self , s : & str) -> Option < String > { use alloc :: vec :: Vec ; use unicode_segmentation :: UnicodeSegmentation ; if let (Some (start) , Some (end)) = (self . start . grapheme_index (s) , self . end . grapheme_index (s)) { Some (s . graphemes (true) . collect :: < Vec < & str > > () [start ..= end] . concat ()) } else { None } } }
};
}
