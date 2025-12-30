// Generated macro for impl_179 (impl)
macro_rules! Depcrate_signaturesimpl_179 {
() => {
// Module: crate::signatures
// Provides: {"impl_179"}
// Dependencies: {}
impl EnumVariants { pub fn variant (& self , name : & Name) -> Option < EnumVariantId > { self . variants . iter () . find_map (| (v , n , _) | if n == name { Some (* v) } else { None }) } pub fn variant_name_by_id (& self , variant_id : EnumVariantId) -> Option < Name > { self . variants . iter () . find_map (| (id , name , _) | if * id == variant_id { Some (name . clone ()) } else { None }) } pub fn is_payload_free (& self , db : & dyn DefDatabase) -> bool { self . variants . iter () . all (| & (v , _ , _) | { let variant = v . fields (db) ; if ! variant . fields () . is_empty () { return false ; } if ! matches ! (variant . shape , FieldsShape :: Unit) { let body = db . body (v . into ()) ; if ! matches ! (body [body . body_expr] , crate :: hir :: Expr :: Missing) { return false ; } } true }) } }
};
}
