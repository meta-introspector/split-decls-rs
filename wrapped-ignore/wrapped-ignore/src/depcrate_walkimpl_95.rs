// Generated macro for impl_95 (impl)
macro_rules! Depcrate_walkimpl_95 {
() => {
// Module: crate::walk
// Provides: {"impl_95"}
// Dependencies: {}
impl std :: fmt :: Debug for WalkBuilder { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("WalkBuilder") . field ("paths" , & self . paths) . field ("ig_builder" , & self . ig_builder) . field ("max_depth" , & self . max_depth) . field ("min_depth" , & self . min_depth) . field ("max_filesize" , & self . max_filesize) . field ("follow_links" , & self . follow_links) . field ("same_file_system" , & self . same_file_system) . field ("sorter" , & "<...>") . field ("threads" , & self . threads) . field ("skip" , & self . skip) . field ("filter" , & "<...>") . field ("global_gitignores_relative_to" , & self . global_gitignores_relative_to ,) . finish () } }
};
}
