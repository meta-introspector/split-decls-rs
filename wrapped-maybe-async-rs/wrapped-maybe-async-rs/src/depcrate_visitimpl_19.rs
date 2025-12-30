// Generated macro for impl_19 (impl)
macro_rules! Depcrate_visitimpl_19 {
() => {
// Module: crate::visit
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > VisitMut for ReplaceGenericType < 'a > { fn visit_item_mut (& mut self , i : & mut Item) { if let Item :: Fn (item_fn) = i { let args = item_fn . sig . generics . params . iter () . filter_map (| param | { if let GenericParam :: Type (type_param) = & param { if type_param . ident . to_string () . eq (self . generic_type) { None } else { Some (param) } } else { Some (param) } }) . collect :: < Vec < _ > > () ; item_fn . sig . generics . params = args . into_iter () . cloned () . collect () ; if let Some (where_clause) = & mut item_fn . sig . generics . where_clause { let new_where_clause = where_clause . predicates . iter () . filter_map (| predicate | { if let WherePredicate :: Type (predicate_type) = predicate { if let Type :: Path (p) = & predicate_type . bounded_ty { if p . path . segments [0] . ident . to_string () . eq (self . generic_type) { None } else { Some (predicate) } } else { Some (predicate) } } else { Some (predicate) } }) . collect :: < Vec < _ > > () ; where_clause . predicates = new_where_clause . into_iter () . cloned () . collect () ; } ; } visit_item_mut (self , i) } fn visit_path_segment_mut (& mut self , i : & mut PathSegment) { if i . ident . to_string () . eq (& self . generic_type) { * i = self . arg_type . clone () ; } visit_path_segment_mut (self , i) ; } }
};
}
