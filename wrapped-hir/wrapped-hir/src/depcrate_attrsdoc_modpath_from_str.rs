// Generated macro for doc_modpath_from_str (function)
macro_rules! Depcrate_attrsdoc_modpath_from_str {
() => {
// Module: crate::attrs
// Provides: {"doc_modpath_from_str"}
// Dependencies: {}
fn doc_modpath_from_str (link : & str) -> Option < ModPath > { let try_get_modpath = | link : & str | { let mut parts = link . split ("::") ; let mut first_segment = None ; let kind = match parts . next () ? { "" => PathKind :: Abs , "crate" => PathKind :: Crate , "self" => PathKind :: SELF , "super" => { let mut deg = 1 ; for segment in parts . by_ref () { if segment == "super" { deg += 1 ; } else { first_segment = Some (segment) ; break ; } } PathKind :: Super (deg) } segment => { first_segment = Some (segment) ; PathKind :: Plain } } ; let parts = first_segment . into_iter () . chain (parts) . map (| segment | match segment . parse () { Ok (idx) => Name :: new_tuple_field (idx) , Err (_) => Name :: new_root (segment . split_once ('<') . map_or (segment , | it | it . 0)) , }) ; Some (ModPath :: from_segments (kind , parts)) } ; try_get_modpath (link) }
};
}
