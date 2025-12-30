// Generated macro for impl_289 (impl)
macro_rules! Depcrate_deriving_generic_tyimpl_289 {
() => {
// Module: crate::deriving::generic::ty
// Provides: {"impl_289"}
// Dependencies: {}
impl Bounds { pub (crate) fn empty () -> Bounds { Bounds { bounds : Vec :: new () } } pub (crate) fn to_generics (& self , cx : & ExtCtxt < '_ > , span : Span , self_ty : Ident , self_generics : & Generics ,) -> Generics { let params = self . bounds . iter () . map (| & (name , ref bounds) | mk_ty_param (cx , span , name , bounds , self_ty , self_generics)) . collect () ; Generics { params , where_clause : ast :: WhereClause { has_where_token : false , predicates : ThinVec :: new () , span , } , span , } } }
};
}
