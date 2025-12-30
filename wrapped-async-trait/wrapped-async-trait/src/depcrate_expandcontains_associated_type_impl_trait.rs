// Generated macro for contains_associated_type_impl_trait (function)
macro_rules! Depcrate_expandcontains_associated_type_impl_trait {
() => {
// Module: crate::expand
// Provides: {"contains_associated_type_impl_trait"}
// Dependencies: {}
fn contains_associated_type_impl_trait (context : Context , ret : & mut Type) -> bool { struct AssociatedTypeImplTraits < 'a > { set : & 'a Set < Ident > , contains : bool , } impl < 'a > VisitMut for AssociatedTypeImplTraits < 'a > { fn visit_type_path_mut (& mut self , ty : & mut TypePath) { if ty . qself . is_none () && ty . path . segments . len () == 2 && ty . path . segments [0] . ident == "Self" && self . set . contains (& ty . path . segments [1] . ident) { self . contains = true ; } visit_mut :: visit_type_path_mut (self , ty) ; } } match context { Context :: Trait { .. } => false , Context :: Impl { associated_type_impl_traits , .. } => { let mut visit = AssociatedTypeImplTraits { set : associated_type_impl_traits , contains : false , } ; visit . visit_type_mut (ret) ; visit . contains } } }
};
}
