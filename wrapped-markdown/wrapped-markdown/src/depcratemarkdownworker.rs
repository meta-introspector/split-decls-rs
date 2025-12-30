// Generated macro for MarkdownWorker (function)
macro_rules! DepcrateMarkdownWorker {
() => {
// Module: crate
// Provides: {"MarkdownWorker"}
// Dependencies: {}
# [oneshot] pub async fn MarkdownWorker (input : String) -> String { let parser = Parser :: new (& input) ; let mut output = String :: new () ; html :: push_html (& mut output , parser) ; output }
};
}
