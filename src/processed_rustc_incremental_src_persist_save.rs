/* FP:save.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0001
/* FP:save.rs-0002 */ use std :: fs ;
/* FP:save.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0002
/* FP:save.rs-0004 */ use std :: sync :: Arc ;
/* FP:save.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0003
/* FP:save.rs-0006 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:save.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0004
/* FP:save.rs-0008 */ use crate :: rustc_data_structures :: sync :: join ;
/* FP:save.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0005
/* FP:save.rs-0010 */ use crate :: rustc_complete :: dep_graph :: { DepGraph , SerializedDepGraph , WorkProduct , WorkProductId , WorkProductMap , } ;
/* FP:save.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0006
/* FP:save.rs-0012 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:save.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0007
/* FP:save.rs-0014 */ use crate :: rustc_serialize :: Encodable as RustcEncodable ;
/* FP:save.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0008
/* FP:save.rs-0016 */ use crate :: rustc_serialize :: opaque :: { FileEncodeResult , FileEncoder } ;
/* FP:save.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0009
/* FP:save.rs-0018 */ use crate :: rustc_complete :: Session ;
/* FP:save.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0010
/* FP:save.rs-0020 */ use tracing :: debug ;
/* FP:save.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0011
/* FP:save.rs-0022 */ use super :: data :: * ;
/* FP:save.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0012
/* FP:save.rs-0024 */ use super :: fs :: * ;
/* FP:save.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0013
/* FP:save.rs-0026 */ use super :: { dirty_clean , file_format , work_product } ;
/* FP:save.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0014
/* FP:save.rs-0028 */ use crate :: assert_dep_graph :: assert_dep_graph ;
/* FP:save.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_USE_0015
/* FP:save.rs-0030 */ use crate :: errors ;
/* FP:save.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_FN_0016
/* FP:save.rs-0032 */ # [doc = " Saves and writes the [`DepGraph`] to the file system."] # [doc = ""] # [doc = " This function saves both the dep-graph and the query result cache,"] # [doc = " and drops the result cache."] # [doc = ""] # [doc = " This function should only run after all queries have completed."] # [doc = " Trying to execute a query afterwards would attempt to read the result cache we just dropped."] pub (crate) fn save_dep_graph (tcx : TyCtxt < '_ >) { debug ! ("save_dep_graph()") ; tcx . dep_graph . with_ignore (| | { let sess = tcx . sess ; if sess . opts . incremental . is_none () { return ; } if sess . dcx () . has_errors_or_delayed_bugs () . is_some () { return ; } let query_cache_path = query_cache_path (sess) ; let dep_graph_path = dep_graph_path (sess) ; let staging_dep_graph_path = staging_dep_graph_path (sess) ; sess . time ("assert_dep_graph" , | | assert_dep_graph (tcx)) ; sess . time ("check_dirty_clean" , | | dirty_clean :: check_dirty_clean_annotations (tcx)) ; join (move | | { sess . time ("incr_comp_persist_dep_graph" , | | { if let Err (err) = fs :: rename (& staging_dep_graph_path , & dep_graph_path) { sess . dcx () . emit_err (errors :: MoveDepGraph { from : & staging_dep_graph_path , to : & dep_graph_path , err , }) ; } }) ; } , move | | { sess . time ("incr_comp_persist_result_cache" , | | { if let Some (odc) = & tcx . query_system . on_disk_cache { odc . drop_serialized_data (tcx) ; } file_format :: save_in (sess , query_cache_path , "query cache" , | e | { encode_query_cache (tcx , e) }) ; }) ; } ,) ; }) }
/* FP:save.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_FN_0017
/* FP:save.rs-0034 */ # [doc = " Saves the work product index."] pub fn save_work_product_index (sess : & Session , dep_graph : & DepGraph , new_work_products : FxIndexMap < WorkProductId , WorkProduct > ,) { if sess . opts . incremental . is_none () { return ; } if sess . dcx () . has_errors () . is_some () { return ; } debug ! ("save_work_product_index()") ; dep_graph . assert_ignored () ; let path = work_products_path (sess) ; file_format :: save_in (sess , path , "work product index" , | mut e | { encode_work_product_index (& new_work_products , & mut e) ; e . finish () }) ; let previous_work_products = dep_graph . previous_work_products () ; for (id , wp) in previous_work_products . to_sorted_stable_ord () { if ! new_work_products . contains_key (id) { work_product :: delete_workproduct_files (sess , wp) ; debug_assert ! (! wp . saved_files . items () . all (| (_ , path) | in_incr_comp_dir_sess (sess , path) . exists ())) ; } } debug_assert ! ({ new_work_products . iter () . all (| (_ , wp) | { wp . saved_files . items () . all (| (_ , path) | in_incr_comp_dir_sess (sess , path) . exists ()) }) }) ; }
/* FP:save.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_FN_0018
/* FP:save.rs-0036 */ fn encode_work_product_index (work_products : & FxIndexMap < WorkProductId , WorkProduct > , encoder : & mut FileEncoder ,) { let serialized_products : Vec < _ > = work_products . iter () . map (| (id , work_product) | SerializedWorkProduct { id : * id , work_product : work_product . clone () , }) . collect () ; serialized_products . encode (encoder) }
/* FP:save.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_FN_0019
/* FP:save.rs-0038 */ fn encode_query_cache (tcx : TyCtxt < '_ > , encoder : FileEncoder) -> FileEncodeResult { tcx . sess . time ("incr_comp_serialize_result_cache" , | | tcx . serialize_query_result_cache (encoder)) }
/* FP:save.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_save_FN_0020
/* FP:save.rs-0040 */ # [doc = " Builds the dependency graph."] # [doc = ""] # [doc = " This function creates the *staging dep-graph*. When the dep-graph is modified by a query"] # [doc = " execution, the new dependency information is not kept in memory but directly"] # [doc = " output to this file. `save_dep_graph` then finalizes the staging dep-graph"] # [doc = " and moves it to the permanent dep-graph path"] pub (crate) fn build_dep_graph (sess : & Session , prev_graph : Arc < SerializedDepGraph > , prev_work_products : WorkProductMap ,) -> Option < DepGraph > { if sess . opts . incremental . is_none () { return None ; } let path_buf = staging_dep_graph_path (sess) ; let mut encoder = match FileEncoder :: new (& path_buf) { Ok (encoder) => encoder , Err (err) => { sess . dcx () . emit_err (errors :: CreateDepGraph { path : & path_buf , err }) ; return None ; } } ; file_format :: write_file_header (& mut encoder , sess) ; sess . opts . dep_tracking_hash (false) . encode (& mut encoder) ; Some (DepGraph :: new (sess , prev_graph , prev_work_products , encoder)) }