// Generated macro for create_runs (function)
macro_rules! Depcrate_runcreate_runs {
() => {
// Module: crate::run
// Provides: {"create_runs"}
// Dependencies: {}
# [test] fn create_runs () { use crate :: font ; use crate :: line :: * ; use crate :: string_attributes :: * ; use core_foundation :: attributed_string :: CFMutableAttributedString ; let mut string = CFMutableAttributedString :: new () ; string . replace_str (& CFString :: new ("Food") , CFRange :: init (0 , 0)) ; let len = string . char_len () ; unsafe { string . set_attribute (CFRange :: init (0 , len) , kCTFontAttributeName , & font :: new_from_name ("Helvetica" , 16.) . unwrap () ,) ; } let line = CTLine :: new_with_attributed_string (string . as_concrete_TypeRef ()) ; let runs = line . glyph_runs () ; assert_eq ! (runs . len () , 1) ; for run in runs . iter () { assert_eq ! (run . glyph_count () , 4) ; let font = run . attributes () . unwrap () . get (CFString :: new ("NSFont")) . downcast :: < font :: CTFont > () . unwrap () ; assert_eq ! (font . pt_size () , 16.) ; let positions = run . positions () ; assert_eq ! (positions . len () , 4) ; assert ! (positions [0] . x < positions [1] . x) ; let glyphs = run . glyphs () ; assert_eq ! (glyphs . len () , 4) ; assert_ne ! (glyphs [0] , glyphs [1]) ; assert_eq ! (glyphs [1] , glyphs [2]) ; let indices = run . string_indices () ; assert_eq ! (indices . as_ref () , & [0 , 1 , 2 , 3]) ; } }
};
}
