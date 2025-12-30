// Generated macro for parser (function)
macro_rules! Depcrateparser {
() => {
// Module: crate
// Provides: {"parser"}
// Dependencies: {}
# [doc = " Convenience function to get a parser matching `mdbook::new_cmark_parser`."] # [doc = ""] # [doc = " This is implemented separately so we are decoupled from mdbook's dependency"] # [doc = " versions and can update at will (albeit with care to stay aligned with what"] # [doc = " mdbook does!) to later versions of `pulldown-cmark` and related tools."] # [doc = ""] # [doc = " Notes:"] # [doc = ""] # [doc = " - `mdbook::new_cmark_parser` has an additional parameter which allows smart"] # [doc = "   punctuation to be enabled or disabled; we always enable it."] # [doc = " - We do not use footnotes in the text at present, but this goes out of its"] # [doc = "   way to match this up to the old footnotes behavior just to make sure the"] # [doc = "   parsing etc. is all the same."] pub fn parser (text : & str) -> Parser < '_ > { let mut opts = Options :: empty () ; opts . insert (Options :: ENABLE_TABLES) ; opts . insert (Options :: ENABLE_FOOTNOTES) ; opts . insert (Options :: ENABLE_OLD_FOOTNOTES) ; opts . insert (Options :: ENABLE_STRIKETHROUGH) ; opts . insert (Options :: ENABLE_TASKLISTS) ; opts . insert (Options :: ENABLE_HEADING_ATTRIBUTES) ; opts . insert (Options :: ENABLE_SMART_PUNCTUATION) ; Parser :: new_ext (text , opts) }
};
}
