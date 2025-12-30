// Generated macro for syntax_tree_enum (function)
macro_rules! Depcrate_debugsyntax_tree_enum {
() => {
// Module: crate::debug
// Provides: {"syntax_tree_enum"}
// Dependencies: {}
fn syntax_tree_enum < 'a > (enum_name : & str , variant_name : & str , fields : & 'a [Type] ,) -> Option < & 'a str > { if fields . len () != 1 { return None ; } const WHITELIST : & [(& str , & str)] = & [("Meta" , "Path") , ("Pat" , "Const") , ("Pat" , "Lit") , ("Pat" , "Macro") , ("Pat" , "Path") , ("Pat" , "Range") , ("PathArguments" , "AngleBracketed") , ("PathArguments" , "Parenthesized") , ("Stmt" , "Local") , ("TypeParamBound" , "Lifetime") , ("Visibility" , "Public") , ("Visibility" , "Restricted") ,] ; match & fields [0] { Type :: Syn (ty) if WHITELIST . contains (& (enum_name , variant_name)) || enum_name . to_owned () + variant_name == * ty => { Some (ty) } _ => None , } }
};
}
