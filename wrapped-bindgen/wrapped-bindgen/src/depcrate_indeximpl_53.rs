// Generated macro for impl_53 (impl)
macro_rules! Depcrate_indeximpl_53 {
() => {
// Module: crate::index
// Provides: {"impl_53"}
// Dependencies: {}
impl Index { fn new () -> Self { Self { version : 2 , .. Default :: default () } } fn get_or_create_namespace_index (& mut self , namespace : & str) -> usize { match self . namespaces . iter () . position (| ns | ns == namespace) { Some (idx) => idx , None => { self . namespaces . push (namespace . to_string ()) ; self . namespaces . len () - 1 } } } fn add_item (& mut self , namespace : & str , name : & str , features : BTreeSet < String >) { let namespace_idx = self . get_or_create_namespace_index (namespace) ; let mut compact = BTreeSet :: new () ; for feature in features . iter () . rev () { if feature . is_empty () { continue ; } if ! compact . iter () . any (| c : & & str | namespace_starts_with (c , feature)) { compact . insert (feature . as_str ()) ; } } let features : Vec < usize > = compact . iter () . map (| dep | self . get_or_create_namespace_index (dep)) . collect () ; let items = self . items . entry (namespace_idx) . or_default () ; if let Some (existing_item) = items . iter_mut () . find (| item | item . name == name) { for & feature in & features { if ! existing_item . features . contains (& feature) { existing_item . features . push (feature) ; } } existing_item . features . sort () ; return ; } let index_item = IndexItem { name : name . to_string () , features , } ; items . push (index_item) ; } }
};
}
