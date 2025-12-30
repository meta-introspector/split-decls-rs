// Generated macro for render_markdown (function)
macro_rules! Depcrate_note_testsrender_markdown {
() => {
// Module: crate::note::tests
// Provides: {"render_markdown"}
// Dependencies: {}
fn render_markdown (text : & str) -> String { let parser = crate :: parser (text) ; let mut buf = String :: new () ; pulldown_cmark :: html :: push_html (& mut buf , parser) ; buf }
};
}
