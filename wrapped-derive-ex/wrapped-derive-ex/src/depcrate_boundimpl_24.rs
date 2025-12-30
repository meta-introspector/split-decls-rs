// Generated macro for impl_24 (impl)
macro_rules! Depcrate_boundimpl_24 {
() => {
// Module: crate::bound
// Provides: {"impl_24"}
// Dependencies: {}
impl WhereClauseBuilder { pub fn new (generics : & Generics) -> Self { let (_ , _ , where_g) = generics . split_for_impl () ; let mut preds = Vec :: new () ; if let Some (where_g) = where_g { preds . extend (where_g . predicates . iter () . cloned ()) ; } Self { types : Vec :: new () , preds , gps : GenericParamSet :: new (generics) , } } pub fn push_bounds (& mut self , bounds : & Bounds) -> bool { self . preds . extend (bounds . pred . iter () . cloned ()) ; self . types . extend (bounds . ty . iter () . cloned ()) ; bounds . default } pub fn push_bounds_for_field (& mut self , field : & Field) { if self . gps . contains_in_type (& field . ty) { self . types . push (field . ty . clone ()) ; } } pub fn build (self , f : impl Fn (& Type) -> TokenStream) -> TokenStream { let mut ws = Vec :: new () ; for ty in & self . types { ws . push (f (ty)) ; } for p in self . preds { ws . push (quote ! (# p)) ; } if ws . is_empty () { quote ! () } else { quote ! (where # (# ws ,) *) } } }
};
}
