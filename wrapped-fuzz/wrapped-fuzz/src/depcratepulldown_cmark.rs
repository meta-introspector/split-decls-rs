// Generated macro for pulldown_cmark (function)
macro_rules! Depcratepulldown_cmark {
() => {
// Module: crate
// Provides: {"pulldown_cmark"}
// Dependencies: {}
# [doc = " Send Markdown `text` to `pulldown-cmark` and return Markdown"] # [doc = " events."] pub fn pulldown_cmark (text : & str) -> Vec < Event < '_ > > { Parser :: new (text) . collect () }
};
}
