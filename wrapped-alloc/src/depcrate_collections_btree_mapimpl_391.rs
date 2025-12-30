// Generated macro for impl_391 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_391 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_391"}
// Dependencies: {}
impl < K , A : Allocator + Clone > BTreeMap < K , SetValZST , A > { pub (super) fn replace (& mut self , key : K) -> Option < K > where K : Ord , { let (map , dormant_map) = DormantMutRef :: new (self) ; let root_node = map . root . get_or_insert_with (| | Root :: new ((* map . alloc) . clone ())) . borrow_mut () ; match root_node . search_tree :: < K > (& key) { Found (mut kv) => Some (mem :: replace (kv . key_mut () , key)) , GoDown (handle) => { VacantEntry { key , handle : Some (handle) , dormant_map , alloc : (* map . alloc) . clone () , _marker : PhantomData , } . insert (SetValZST) ; None } } } pub (super) fn get_or_insert_with < Q : ? Sized , F > (& mut self , q : & Q , f : F) -> & K where K : Borrow < Q > + Ord , Q : Ord , F : FnOnce (& Q) -> K , { let (map , dormant_map) = DormantMutRef :: new (self) ; let root_node = map . root . get_or_insert_with (| | Root :: new ((* map . alloc) . clone ())) . borrow_mut () ; match root_node . search_tree (q) { Found (handle) => handle . into_kv_mut () . 0 , GoDown (handle) => { let key = f (q) ; assert ! (* key . borrow () == * q , "new value is not equal") ; VacantEntry { key , handle : Some (handle) , dormant_map , alloc : (* map . alloc) . clone () , _marker : PhantomData , } . insert_entry (SetValZST) . into_key () } } } }
};
}
