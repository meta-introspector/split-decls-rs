// Generated macro for fetch_attr (function)
macro_rules! Depcratefetch_attr {
() => {
// Module: crate
// Provides: {"fetch_attr"}
// Dependencies: {}
# [doc = " Fetch an attribute string from the derived struct."] fn fetch_attr (name : & str , attrs : & [syn :: Attribute]) -> Option < String > { for attr in attrs { if let Ok (meta) = attr . parse_meta () { match meta { syn :: Meta :: NameValue (nv) => { if nv . path . get_ident () . map (| i | i . to_string ()) == Some (name . to_string ()) { match nv . lit { syn :: Lit :: Str (ref s) => return Some (s . value ()) , _ => { panic ! ("attribute {} should be a string" , name) ; } } } } _ => { panic ! ("attribute {} should be a string" , name) ; } } } } None }
};
}
