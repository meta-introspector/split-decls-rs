// Generated macro for Parser (struct)
macro_rules! Depcrate_parseParser {
() => {
// Module: crate::parse
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " Markdown event iterator."] pub struct Parser < 'input , F = DefaultBrokenLinkCallback > { broken_link_callback : Option < F > , inner : ParserInner < 'input > , }
};
}
