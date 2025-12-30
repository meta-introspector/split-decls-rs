// Generated macro for ends_with_line_comment_or_broken (function)
macro_rules! Depcrate_sourceends_with_line_comment_or_broken {
() => {
// Module: crate::source
// Provides: {"ends_with_line_comment_or_broken"}
// Dependencies: {}
fn ends_with_line_comment_or_broken (text : & str) -> bool { let Some (last) = tokenize (text , FrontmatterAllowed :: No) . last () else { return false ; } ; match last . kind { TokenKind :: LineComment { .. } | TokenKind :: BlockComment { terminated : false , .. } => true , TokenKind :: Literal { kind , .. } => matches ! (kind , LiteralKind :: Byte { terminated : false } | LiteralKind :: ByteStr { terminated : false } | LiteralKind :: CStr { terminated : false } | LiteralKind :: Char { terminated : false } | LiteralKind :: RawByteStr { n_hashes : None } | LiteralKind :: RawCStr { n_hashes : None } | LiteralKind :: RawStr { n_hashes : None }) , _ => false , } }
};
}
