// Generated macro for dewhereselfify (function)
macro_rules! Depcratedewhereselfify {
() => {
// Module: crate
// Provides: {"dewhereselfify"}
// Dependencies: {}
# [doc = " Remove any generics that place constraints on Self."] fn dewhereselfify (generics : & mut Generics) { if let Some (ref mut wc) = & mut generics . where_clause { let new_predicates = wc . predicates . iter () . filter (| wp | match wp { WherePredicate :: Type (pt) => { pt . bounded_ty != parse2 (quote ! (Self)) . unwrap () } , _ => true }) . cloned () . collect :: < Punctuated < WherePredicate , Token ! [,] > > () ; wc . predicates = new_predicates ; } if generics . where_clause . as_ref () . map (| wc | wc . predicates . is_empty ()) . unwrap_or (false) { generics . where_clause = None ; } }
};
}
