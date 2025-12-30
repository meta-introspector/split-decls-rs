// Generated macro for AllObjects (struct)
macro_rules! Depcrate_store_impls_dynamic_iterAllObjects {
() => {
// Module: crate::store_impls::dynamic::iter
// Provides: {"AllObjects"}
// Dependencies: {}
# [doc = " An iterator over all, _possibly duplicate_, objects of an object store, which by default uses no extra memory but yields an"] # [doc = " order that is costly to traverse when querying object information or decoding them."] # [doc = ""] # [doc = " Use [`with_ordering()`][AllObjects::with_ordering()] to choose a performance trade-off."] pub struct AllObjects { state : State , num_objects : usize , loose_dbs : Arc < Vec < loose :: Store > > , order : Ordering , }
};
}
