// Generated macro for delete_dirty_work_product (function)
macro_rules! Depcrate_persist_loaddelete_dirty_work_product {
() => {
// Module: crate::persist::load
// Provides: {"delete_dirty_work_product"}
// Dependencies: {}
fn delete_dirty_work_product (sess : & Session , swp : SerializedWorkProduct) { debug ! ("delete_dirty_work_product({:?})" , swp) ; work_product :: delete_workproduct_files (sess , & swp . work_product) ; }
};
}
