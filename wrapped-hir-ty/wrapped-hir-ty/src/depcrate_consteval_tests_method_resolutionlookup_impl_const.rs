// Generated macro for lookup_impl_const (function)
macro_rules! Depcrate_consteval_tests_method_resolutionlookup_impl_const {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"lookup_impl_const"}
// Dependencies: {}
pub fn lookup_impl_const < 'db > (infcx : & InferCtxt < 'db > , env : Arc < TraitEnvironment < 'db > > , const_id : ConstId , subs : GenericArgs < 'db > ,) -> (ConstId , GenericArgs < 'db >) { let interner = infcx . interner ; let db = interner . db ; let trait_id = match const_id . loc (db) . container { ItemContainerId :: TraitId (id) => id , _ => return (const_id , subs) , } ; let trait_ref = TraitRef :: new (interner , trait_id . into () , subs) ; let const_signature = db . const_signature (const_id) ; let name = match const_signature . name . as_ref () { Some (name) => name , None => return (const_id , subs) , } ; lookup_impl_assoc_item_for_trait_ref (infcx , trait_ref , env , name) . and_then (| assoc | if let (AssocItemId :: ConstId (id) , s) = assoc { Some ((id , s)) } else { None } ,) . unwrap_or ((const_id , subs)) }
};
}
