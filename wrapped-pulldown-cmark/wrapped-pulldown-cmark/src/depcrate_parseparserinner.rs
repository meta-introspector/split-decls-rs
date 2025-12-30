// Generated macro for ParserInner (struct)
macro_rules! Depcrate_parseParserInner {
() => {
// Module: crate::parse
// Provides: {"ParserInner"}
// Dependencies: {}
struct ParserInner < 'input > { text : & 'input str , options : Options , tree : Tree < Item > , allocs : Allocations < 'input > , html_scan_guard : HtmlScanGuard , link_ref_expansion_limit : usize , inline_stack : InlineStack , link_stack : LinkStack , wikilink_stack : LinkStack , code_delims : CodeDelims , math_delims : MathDelims , }
};
}
