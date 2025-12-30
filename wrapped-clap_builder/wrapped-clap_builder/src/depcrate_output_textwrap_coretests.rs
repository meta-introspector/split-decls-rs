// Generated macro for tests (module)
macro_rules! Depcrate_output_textwrap_coretests {
() => {
// Module: crate::output::textwrap::core
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [cfg (feature = "unicode")] use unicode_width :: UnicodeWidthChar ; # [test] fn emojis_have_correct_width () { use unic_emoji_char :: is_emoji ; for ch in '\u{1}' .. '\u{FF}' { if is_emoji (ch) { let desc = format ! ("{:?} U+{:04X}" , ch , ch as u32) ; # [cfg (feature = "unicode")] assert_eq ! (ch . width () . unwrap () , 1 , "char: {desc}") ; # [cfg (not (feature = "unicode"))] assert_eq ! (ch_width (ch) , 1 , "char: {desc}") ; } } for ch in '\u{FF}' .. '\u{2FFFF}' { if is_emoji (ch) { let desc = format ! ("{:?} U+{:04X}" , ch , ch as u32) ; # [cfg (feature = "unicode")] assert ! (ch . width () . unwrap () <= 2 , "char: {desc}") ; # [cfg (not (feature = "unicode"))] assert_eq ! (ch_width (ch) , 1 , "char: {desc}") ; } } } # [test] # [cfg (feature = "unicode")] fn display_width_works () { assert_eq ! ("Café Plain" . len () , 11) ; assert_eq ! (display_width ("Café Plain") , 10) ; } # [test] # [cfg (feature = "unicode")] fn display_width_narrow_emojis () { assert_eq ! (display_width ("⁉") , 1) ; } # [test] # [cfg (feature = "unicode")] fn display_width_narrow_emojis_variant_selector () { assert_eq ! (display_width ("⁉\u{fe0f}") , 1) ; } # [test] # [cfg (feature = "unicode")] fn display_width_emojis () { assert_eq ! (display_width ("😂😭🥺🤣✨😍🙏🥰😊🔥") , 20) ; } }
};
}
