// Generated macro for test (module)
macro_rules! Depcrate_style_font_ttftest {
() => {
// Module: crate::style::font::ttf
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_font_cache () -> FontResult < () > { let _a = load_font_data (FontFamily :: Serif , FontStyle :: Normal) ? ; assert ! (DATA_CACHE . read () . unwrap () . contains_key ("serif")) ; let _b = load_font_data (FontFamily :: Serif , FontStyle :: Normal) ? ; assert ! (DATA_CACHE . read () . unwrap () . contains_key ("serif")) ; Ok (()) } }
};
}
