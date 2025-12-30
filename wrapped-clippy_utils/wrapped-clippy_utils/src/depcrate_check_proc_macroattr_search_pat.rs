// Generated macro for attr_search_pat (function)
macro_rules! Depcrate_check_proc_macroattr_search_pat {
() => {
// Module: crate::check_proc_macro
// Provides: {"attr_search_pat"}
// Dependencies: {}
fn attr_search_pat (attr : & Attribute) -> (Pat , Pat) { match attr . kind { AttrKind :: Normal (..) => { if let Some (ident) = attr . ident () { let ident_string = ident . to_string () ; if attr . style == AttrStyle :: Outer { (Pat :: OwnedMultiStr (vec ! ["#[" . to_owned () + & ident_string , ident_string]) , Pat :: Str ("") ,) } else { (Pat :: OwnedMultiStr (vec ! ["#![" . to_owned () + & ident_string , ident_string]) , Pat :: Str ("") ,) } } else { (Pat :: Str ("#") , Pat :: Str ("]")) } } , AttrKind :: DocComment (_kind @ CommentKind :: Line , ..) => { if attr . style == AttrStyle :: Outer { (Pat :: Str ("///") , Pat :: Str ("")) } else { (Pat :: Str ("//!") , Pat :: Str ("")) } } , AttrKind :: DocComment (_kind @ CommentKind :: Block , ..) => { if attr . style == AttrStyle :: Outer { (Pat :: Str ("/**") , Pat :: Str ("*/")) } else { (Pat :: Str ("/*!") , Pat :: Str ("*/")) } } , } }
};
}
