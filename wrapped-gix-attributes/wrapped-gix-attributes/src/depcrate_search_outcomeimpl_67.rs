// Generated macro for impl_67 (impl)
macro_rules! Depcrate_search_outcomeimpl_67 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_67"}
// Dependencies: {}
impl MetadataCollection { pub (crate) fn id_for_macro (& mut self , name : & str , attrs : & mut Assignments) -> AttributeId { let order = match self . name_to_meta . get_mut (name) { Some (meta) => meta . id , None => { let order = AttributeId (self . name_to_meta . len ()) ; self . name_to_meta . insert (KString :: from_ref (name) , Metadata { id : order , macro_attributes : Default :: default () , } ,) ; order } } ; self . assign_order_to_attributes (attrs) ; self . name_to_meta . get_mut (name) . expect ("just added") . macro_attributes . clone_from (attrs) ; order } pub (crate) fn id_for_attribute (& mut self , name : & str) -> AttributeId { match self . name_to_meta . get (name) { Some (meta) => meta . id , None => { let order = AttributeId (self . name_to_meta . len ()) ; self . name_to_meta . insert (KString :: from_ref (name) , order . into ()) ; order } } } pub (crate) fn assign_order_to_attributes (& mut self , attributes : & mut [TrackedAssignment]) { for TrackedAssignment { id : order , inner : crate :: Assignment { name , .. } , } in attributes { * order = self . id_for_attribute (& name . 0) ; } } }
};
}
