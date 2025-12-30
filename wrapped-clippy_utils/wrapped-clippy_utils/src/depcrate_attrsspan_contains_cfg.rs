// Generated macro for span_contains_cfg (function)
macro_rules! Depcrate_attrsspan_contains_cfg {
() => {
// Module: crate::attrs
// Provides: {"span_contains_cfg"}
// Dependencies: {}
# [doc = " Checks whether the given span contains a `#[cfg(..)]` attribute"] pub fn span_contains_cfg (cx : & LateContext < '_ > , s : Span) -> bool { s . check_source_text (cx , | src | { let mut iter = tokenize_with_text (src) ; while iter . any (| (t , ..) | matches ! (t , TokenKind :: Pound)) { let mut iter = iter . by_ref () . skip_while (| (t , ..) | { matches ! (t , TokenKind :: Whitespace | TokenKind :: LineComment { .. } | TokenKind :: BlockComment { .. }) }) ; if matches ! (iter . next () , Some ((TokenKind :: OpenBracket , ..))) && matches ! (iter . next () , Some ((TokenKind :: Ident , "cfg" , _))) { return true ; } } false }) }
};
}
