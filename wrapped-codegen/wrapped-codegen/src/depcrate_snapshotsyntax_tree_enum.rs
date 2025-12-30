// Generated macro for syntax_tree_enum (function)
macro_rules! Depcrate_snapshotsyntax_tree_enum {
() => {
// Module: crate::snapshot
// Provides: {"syntax_tree_enum"}
// Dependencies: {}
fn syntax_tree_enum < 'a > (outer : & str , inner : & str , fields : & 'a [Type]) -> Option < & 'a str > { if fields . len () != 1 { return None ; } const WHITELIST : & [(& str , & str)] = & [("Meta" , "Path") , ("PathArguments" , "AngleBracketed") , ("PathArguments" , "Parenthesized") , ("Stmt" , "Local") , ("TypeParamBound" , "Lifetime") , ("Visibility" , "Public") , ("Visibility" , "Restricted") ,] ; match & fields [0] { Type :: Syn (ty) if WHITELIST . contains (& (outer , inner)) || outer . to_owned () + inner == * ty => { Some (ty) } _ => None , } }
};
}
