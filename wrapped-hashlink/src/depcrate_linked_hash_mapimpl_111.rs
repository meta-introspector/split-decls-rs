// Generated macro for impl_111 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_111 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_111"}
// Dependencies: {}
impl < K , V , S > IntoIterator for LinkedHashMap < K , V , S > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (mut self) -> IntoIter < K , V > { unsafe { let (head , tail) = if let Some (values) = self . values { let ValueLinks { next : head , prev : tail , } = values . as_ref () . links . value ; let _ = Box :: from_raw (self . values . as_ptr ()) ; self . values = None ; (Some (head) , Some (tail)) } else { (None , None) } ; let len = self . len () ; drop_free_nodes (self . free . take ()) ; self . table . clear () ; IntoIter { head , tail , remaining : len , marker : PhantomData , } } } }
};
}
