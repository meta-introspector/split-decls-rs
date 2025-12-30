// Generated macro for save_work_product_index (function)
macro_rules! Depcrate_persist_savesave_work_product_index {
() => {
// Module: crate::persist::save
// Provides: {"save_work_product_index"}
// Dependencies: {}
# [doc = " Saves the work product index."] pub fn save_work_product_index (sess : & Session , dep_graph : & DepGraph , new_work_products : FxIndexMap < WorkProductId , WorkProduct > ,) { if sess . opts . incremental . is_none () { return ; } if sess . dcx () . has_errors () . is_some () { return ; } debug ! ("save_work_product_index()") ; dep_graph . assert_ignored () ; let path = work_products_path (sess) ; file_format :: save_in (sess , path , "work product index" , | mut e | { encode_work_product_index (& new_work_products , & mut e) ; e . finish () }) ; let previous_work_products = dep_graph . previous_work_products () ; for (id , wp) in previous_work_products . to_sorted_stable_ord () { if ! new_work_products . contains_key (id) { work_product :: delete_workproduct_files (sess , wp) ; debug_assert ! (! wp . saved_files . items () . all (| (_ , path) | in_incr_comp_dir_sess (sess , path) . exists ())) ; } } debug_assert ! ({ new_work_products . iter () . all (| (_ , wp) | { wp . saved_files . items () . all (| (_ , path) | in_incr_comp_dir_sess (sess , path) . exists ()) }) }) ; }
};
}
