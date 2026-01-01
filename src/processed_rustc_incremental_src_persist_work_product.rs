/* FP:work_product.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0001
/* FP:work_product.rs-0002 */ use std :: fs as std_fs ;
/* FP:work_product.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0002
/* FP:work_product.rs-0004 */ use std :: path :: { Path , PathBuf } ;
/* FP:work_product.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0003
/* FP:work_product.rs-0006 */ use crate :: rustc_data_structures :: unord :: UnordMap ;
/* FP:work_product.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0004
/* FP:work_product.rs-0008 */ use rustc_fs_util :: link_or_copy ;
/* FP:work_product.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0005
/* FP:work_product.rs-0010 */ use crate :: rustc_complete :: dep_graph :: { WorkProduct , WorkProductId } ;
/* FP:work_product.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0006
/* FP:work_product.rs-0012 */ use crate :: rustc_complete :: Session ;
/* FP:work_product.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0007
/* FP:work_product.rs-0014 */ use tracing :: debug ;
/* FP:work_product.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0008
/* FP:work_product.rs-0016 */ use crate :: errors ;
/* FP:work_product.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_USE_0009
/* FP:work_product.rs-0018 */ use crate :: persist :: fs :: * ;
/* FP:work_product.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_FN_0010
/* FP:work_product.rs-0020 */ # [doc = " Copies a CGU work product to the incremental compilation directory, so next compilation can"] # [doc = " find and reuse it."] pub fn copy_cgu_workproduct_to_incr_comp_cache_dir (sess : & Session , cgu_name : & str , files : & [(& 'static str , & Path)] , known_links : & [PathBuf] ,) -> Option < (WorkProductId , WorkProduct) > { debug ! (? cgu_name , ? files) ; sess . opts . incremental . as_ref () ? ; let mut saved_files = UnordMap :: default () ; for (ext , path) in files { let file_name = format ! ("{cgu_name}.{ext}") ; let path_in_incr_dir = in_incr_comp_dir_sess (sess , & file_name) ; if known_links . contains (& path_in_incr_dir) { let _ = saved_files . insert (ext . to_string () , file_name) ; continue ; } match link_or_copy (path , & path_in_incr_dir) { Ok (_) => { let _ = saved_files . insert (ext . to_string () , file_name) ; } Err (err) => { sess . dcx () . emit_warn (errors :: CopyWorkProductToCache { from : path , to : & path_in_incr_dir , err , }) ; } } } let work_product = WorkProduct { cgu_name : cgu_name . to_string () , saved_files } ; debug ! (? work_product) ; let work_product_id = WorkProductId :: from_cgu_name (cgu_name) ; Some ((work_product_id , work_product)) }
/* FP:work_product.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_incremental_src_persist_work_product_FN_0011
/* FP:work_product.rs-0022 */ # [doc = " Removes files for a given work product."] pub (crate) fn delete_workproduct_files (sess : & Session , work_product : & WorkProduct) { for (_ , path) in work_product . saved_files . items () . into_sorted_stable_ord () { let path = in_incr_comp_dir_sess (sess , path) ; if let Err (err) = std_fs :: remove_file (& path) { sess . dcx () . emit_warn (errors :: DeleteWorkProduct { path : & path , err }) ; } } }