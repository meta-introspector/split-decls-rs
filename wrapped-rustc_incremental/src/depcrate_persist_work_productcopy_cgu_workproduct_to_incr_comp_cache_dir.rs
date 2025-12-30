// Generated macro for copy_cgu_workproduct_to_incr_comp_cache_dir (function)
macro_rules! Depcrate_persist_work_productcopy_cgu_workproduct_to_incr_comp_cache_dir {
() => {
// Module: crate::persist::work_product
// Provides: {"copy_cgu_workproduct_to_incr_comp_cache_dir"}
// Dependencies: {}
# [doc = " Copies a CGU work product to the incremental compilation directory, so next compilation can"] # [doc = " find and reuse it."] pub fn copy_cgu_workproduct_to_incr_comp_cache_dir (sess : & Session , cgu_name : & str , files : & [(& 'static str , & Path)] , known_links : & [PathBuf] ,) -> Option < (WorkProductId , WorkProduct) > { debug ! (? cgu_name , ? files) ; sess . opts . incremental . as_ref () ? ; let mut saved_files = UnordMap :: default () ; for (ext , path) in files { let file_name = format ! ("{cgu_name}.{ext}") ; let path_in_incr_dir = in_incr_comp_dir_sess (sess , & file_name) ; if known_links . contains (& path_in_incr_dir) { let _ = saved_files . insert (ext . to_string () , file_name) ; continue ; } match link_or_copy (path , & path_in_incr_dir) { Ok (_) => { let _ = saved_files . insert (ext . to_string () , file_name) ; } Err (err) => { sess . dcx () . emit_warn (errors :: CopyWorkProductToCache { from : path , to : & path_in_incr_dir , err , }) ; } } } let work_product = WorkProduct { cgu_name : cgu_name . to_string () , saved_files } ; debug ! (? work_product) ; let work_product_id = WorkProductId :: from_cgu_name (cgu_name) ; Some ((work_product_id , work_product)) }
};
}
