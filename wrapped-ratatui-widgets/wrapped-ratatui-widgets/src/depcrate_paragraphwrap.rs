// Generated macro for Wrap (struct)
macro_rules! Depcrate_paragraphWrap {
() => {
// Module: crate::paragraph
// Provides: {"Wrap"}
// Dependencies: {}
# [doc = " Describes how to wrap text across lines."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::text::Text;"] # [doc = " use ratatui::widgets::{Paragraph, Wrap};"] # [doc = ""] # [doc = " let bullet_points = Text::from("] # [doc = "     r#\"Some indented points:"] # [doc = "     - First thing goes here and is long so that it wraps"] # [doc = "     - Here is another point that is long enough to wrap\"#,"] # [doc = " );"] # [doc = ""] # [doc = " // With leading spaces trimmed (window width of 30 chars):"] # [doc = " Paragraph::new(bullet_points.clone()).wrap(Wrap { trim: true });"] # [doc = " // Some indented points:"] # [doc = " // - First thing goes here and is"] # [doc = " // long so that it wraps"] # [doc = " // - Here is another point that"] # [doc = " // is long enough to wrap"] # [doc = ""] # [doc = " // But without trimming, indentation is preserved:"] # [doc = " Paragraph::new(bullet_points).wrap(Wrap { trim: false });"] # [doc = " // Some indented points:"] # [doc = " //     - First thing goes here"] # [doc = " // and is long so that it wraps"] # [doc = " //     - Here is another point"] # [doc = " // that is long enough to wrap"] # [doc = " ```"] # [derive (Debug , Default , Clone , Copy , Eq , PartialEq , Hash)] pub struct Wrap { # [doc = " Should leading whitespace be trimmed"] pub trim : bool , }
};
}
