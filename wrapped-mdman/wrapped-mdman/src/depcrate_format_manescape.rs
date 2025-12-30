// Generated macro for escape (function)
macro_rules! Depcrate_format_manescape {
() => {
// Module: crate::format::man
// Provides: {"escape"}
// Dependencies: {}
# [allow (clippy :: collapsible_str_replace)] fn escape (s : & str) -> Result < String , Error > { let mut replaced = s . replace ('\\' , "\\(rs") . replace ('-' , "\\-") . replace ('\u{00A0}' , "\\ ") . replace ('–' , "\\[en]") . replace ('—' , "\\[em]") . replace ('‘' , "\\[oq]") . replace ('’' , "\\[cq]") . replace ('“' , "\\[lq]") . replace ('”' , "\\[rq]") . replace ('…' , "\\[u2026]") . replace ('│' , "|") . replace ('├' , "|") . replace ('└' , "`") . replace ('─' , "\\-") ; if replaced . starts_with ('.') { replaced = format ! ("\\&.{}" , & replaced [1 ..]) ; } if let Some (ch) = replaced . chars () . find (| ch | { ! matches ! (ch , '\n' | ' ' | '!' ..='/' | '0' ..='9' | ':' ..='@' | 'A' ..='Z' | '[' ..='`' | 'a' ..='z' | '{' ..='~') }) { bail ! ("character {:?} is not allowed (update the translation table if needed)" , ch) ; } Ok (replaced) }
};
}
