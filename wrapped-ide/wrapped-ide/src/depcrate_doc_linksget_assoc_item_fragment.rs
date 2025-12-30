// Generated macro for get_assoc_item_fragment (function)
macro_rules! Depcrate_doc_linksget_assoc_item_fragment {
() => {
// Module: crate::doc_links
// Provides: {"get_assoc_item_fragment"}
// Dependencies: {}
# [doc = " Get the fragment required to link to a specific field, method, associated type, or associated constant."] # [doc = ""] # [doc = " ```ignore"] # [doc = " https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next"] # [doc = "                                                       ^^^^^^^^^^^^^^"] # [doc = " ```"] fn get_assoc_item_fragment (db : & dyn HirDatabase , assoc_item : hir :: AssocItem) -> Option < String > { Some (match assoc_item { AssocItem :: Function (function) => { let is_trait_method = function . as_assoc_item (db) . and_then (| assoc | assoc . container_trait (db)) . is_some () ; if is_trait_method && ! function . has_body (db) { format ! ("tymethod.{}" , function . name (db) . as_str ()) } else { format ! ("method.{}" , function . name (db) . as_str ()) } } AssocItem :: Const (constant) => { format ! ("associatedconstant.{}" , constant . name (db) ?. as_str ()) } AssocItem :: TypeAlias (ty) => { format ! ("associatedtype.{}" , ty . name (db) . as_str ()) } }) }
};
}
