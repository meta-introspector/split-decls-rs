// Generated macro for data_id_for_alloc_id (function)
macro_rules! Depcrate_constantdata_id_for_alloc_id {
() => {
// Module: crate::constant
// Provides: {"data_id_for_alloc_id"}
// Dependencies: {}
fn data_id_for_alloc_id (cx : & mut ConstantCx , module : & mut dyn Module , alloc_id : AllocId , mutability : rustc_hir :: Mutability ,) -> DataId { cx . todo . push (TodoItem :: Alloc (alloc_id)) ; * cx . anon_allocs . entry (alloc_id) . or_insert_with (| | module . declare_anonymous_data (mutability . is_mut () , false) . unwrap ()) }
};
}
