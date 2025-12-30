// Generated macro for Parser (struct)
macro_rules! Depcrate_hir_parseParser {
() => {
// Module: crate::hir::parse
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A regular expression parser."] # [doc = ""] # [doc = " This parses a string representation of a regular expression into an"] # [doc = " abstract syntax tree. The size of the tree is proportional to the length"] # [doc = " of the regular expression pattern."] # [doc = ""] # [doc = " A `Parser` can be configured in more detail via a [`ParserBuilder`]."] # [derive (Clone , Debug)] pub (super) struct Parser < 'a > { # [doc = " The configuration of the parser as given by the caller."] config : Config , # [doc = " The pattern we're parsing as given by the caller."] pattern : & 'a str , # [doc = " The call depth of the parser. This is incremented for each"] # [doc = " sub-expression parsed. Its peak value is the maximum nesting of the"] # [doc = " pattern."] depth : Cell < u32 > , # [doc = " The current position of the parser."] pos : Cell < usize > , # [doc = " The current codepoint of the parser. The codepoint corresponds to the"] # [doc = " codepoint encoded in `pattern` beginning at `pos`."] # [doc = ""] # [doc = " This is `None` if and only if `pos == pattern.len()`."] char : Cell < Option < char > > , # [doc = " The current capture index."] capture_index : Cell < u32 > , # [doc = " The flags that are currently set."] flags : RefCell < Flags > , # [doc = " A sorted sequence of capture names. This is used to detect duplicate"] # [doc = " capture names and report an error if one is detected."] capture_names : RefCell < Vec < String > > , }
};
}
