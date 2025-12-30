// Generated macro for SyntaxError (enum)
macro_rules! Depcrate_errorsSyntaxError {
() => {
// Module: crate::errors
// Provides: {"SyntaxError"}
// Dependencies: {}
# [doc = " An error returned if parsed document does not correspond to the XML grammar,"] # [doc = " for example, a tag opened by `<` not closed with `>`. This error does not"] # [doc = " represent invalid XML constructs, for example, tags `<>` and `</>` a well-formed"] # [doc = " from syntax point-of-view."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum SyntaxError { # [doc = " The parser started to parse `<!`, but the input ended before it can recognize"] # [doc = " anything."] InvalidBangMarkup , # [doc = " The parser started to parse processing instruction or XML declaration (`<?`),"] # [doc = " but the input ended before the `?>` sequence was found."] UnclosedPIOrXmlDecl , # [doc = " The parser started to parse comment (`<!--`) content, but the input ended"] # [doc = " before the `-->` sequence was found."] UnclosedComment , # [doc = " The parser started to parse DTD (`<!DOCTYPE`) content, but the input ended"] # [doc = " before the closing `>` character was found."] UnclosedDoctype , # [doc = " The parser started to parse `<![CDATA[` content, but the input ended"] # [doc = " before the `]]>` sequence was found."] UnclosedCData , # [doc = " The parser started to parse tag content, but the input ended"] # [doc = " before the closing `>` character was found."] UnclosedTag , }
};
}
