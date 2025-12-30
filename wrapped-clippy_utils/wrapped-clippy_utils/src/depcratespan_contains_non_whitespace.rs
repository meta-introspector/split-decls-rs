// Generated macro for span_contains_non_whitespace (function)
macro_rules! Depcratespan_contains_non_whitespace {
() => {
// Module: crate
// Provides: {"span_contains_non_whitespace"}
// Dependencies: {}
# [doc = " Checks whether a given span has any significant token. A significant token is a non-whitespace"] # [doc = " token, including comments unless `skip_comments` is set."] # [doc = " This is useful to determine if there are any actual code tokens in the span that are omitted in"] # [doc = " the late pass, such as platform-specific code."] pub fn span_contains_non_whitespace (cx : & impl source :: HasSession , span : Span , skip_comments : bool) -> bool { matches ! (span . get_source_text (cx) , Some (snippet) if tokenize_with_text (& snippet) . any (| (token , _ , _) | match token { TokenKind :: Whitespace => false , TokenKind :: BlockComment { .. } | TokenKind :: LineComment { .. } => ! skip_comments , _ => true , })) }
};
}
