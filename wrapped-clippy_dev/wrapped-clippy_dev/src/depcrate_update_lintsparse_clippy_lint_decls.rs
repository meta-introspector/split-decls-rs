// Generated macro for parse_clippy_lint_decls (function)
macro_rules! Depcrate_update_lintsparse_clippy_lint_decls {
() => {
// Module: crate::update_lints
// Provides: {"parse_clippy_lint_decls"}
// Dependencies: {}
# [doc = " Parse a source file looking for `declare_clippy_lint` macro invocations."] fn parse_clippy_lint_decls (path : & Path , contents : & str , module : & str , lints : & mut Vec < Lint >) { # [allow (clippy :: enum_glob_use)] use Token :: * ; # [rustfmt :: skip] static DECL_TOKENS : & [Token < '_ >] = & [Bang , OpenBrace , AnyComment , Pound , OpenBracket , Ident ("clippy") , DoubleColon , Ident ("version") , Eq , LitStr , CloseBracket , Ident ("pub") , CaptureIdent , Comma , AnyComment , CaptureIdent , Comma ,] ; let mut searcher = RustSearcher :: new (contents) ; while searcher . find_token (Ident ("declare_clippy_lint")) { let start = searcher . pos () as usize - "declare_clippy_lint" . len () ; let (mut name , mut group) = ("" , "") ; if searcher . match_tokens (DECL_TOKENS , & mut [& mut name , & mut group]) && searcher . find_token (CloseBrace) { lints . push (Lint { name : name . to_lowercase () , group : group . into () , module : module . into () , path : path . into () , declaration_range : start .. searcher . pos () as usize , }) ; } } }
};
}
