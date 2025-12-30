// Generated macro for parse_mod_file (function)
macro_rules! Depcrate_new_lintparse_mod_file {
() => {
// Module: crate::new_lint
// Provides: {"parse_mod_file"}
// Dependencies: {}
fn parse_mod_file (path : & Path , contents : & str) -> (& 'static str , usize) { # [allow (clippy :: enum_glob_use)] use cursor :: Pat :: * ; let mut context = None ; let mut decl_end = None ; let mut cursor = Cursor :: new (contents) ; let mut captures = [Capture :: EMPTY] ; while let Some (name) = cursor . find_any_ident () { match cursor . get_text (name) { "declare_clippy_lint" => { if cursor . match_all (& [Bang , OpenBrace] , & mut []) && cursor . find_pat (CloseBrace) { decl_end = Some (cursor . pos ()) ; } } , "impl" => { if cursor . match_all (& [Lt , Lifetime , Gt , CaptureIdent] , & mut captures) { match cursor . get_text (captures [0]) { "LateLintPass" => context = Some ("LateContext") , "EarlyLintPass" => context = Some ("EarlyContext") , _ => { } , } } } , _ => { } , } } (context . unwrap_or_else (| | panic ! ("No lint pass implementation found in `{}`" , path . display ())) , decl_end . unwrap_or_else (| | panic ! ("No lint declarations found in `{}`" , path . display ())) as usize ,) }
};
}
