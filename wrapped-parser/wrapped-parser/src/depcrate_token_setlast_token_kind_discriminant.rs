// Generated macro for LAST_TOKEN_KIND_DISCRIMINANT (const)
macro_rules! Depcrate_token_setLAST_TOKEN_KIND_DISCRIMINANT {
() => {
// Module: crate::token_set
// Provides: {"LAST_TOKEN_KIND_DISCRIMINANT"}
// Dependencies: {}
# [doc = " `TokenSet`s should only include token `SyntaxKind`s, so the discriminant of any passed/included"] # [doc = " `SyntaxKind` must *not* be greater than that of the last token `SyntaxKind`."] # [doc = " See #17037."] const LAST_TOKEN_KIND_DISCRIMINANT : usize = SyntaxKind :: SHEBANG as usize ;
};
}
