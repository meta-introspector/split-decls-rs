// Generated macro for finalize (function)
macro_rules! Depcrate_coverageinfo_mapgenfinalize {
() => {
// Module: crate::coverageinfo::mapgen
// Provides: {"finalize"}
// Dependencies: {}
# [doc = " Generates and exports the coverage map, which is embedded in special"] # [doc = " linker sections in the final binary."] # [doc = ""] # [doc = " Those sections are then read and understood by LLVM's `llvm-cov` tool,"] # [doc = " which is distributed in the `llvm-tools` rustup component."] pub (crate) fn finalize (cx : & mut CodegenCx < '_ , '_ >) { let tcx = cx . tcx ; let covmap_version = CovmapVersion :: try_from (llvm_cov :: mapping_version ()) . unwrap_or_else (| raw_version : u32 | { panic ! ("unknown coverage mapping version reported by `llvm-wrapper`: {raw_version}") }) ; assert_matches ! (covmap_version , CovmapVersion :: Version7) ; debug ! ("Generating coverage map for CodegenUnit: `{}`" , cx . codegen_unit . name ()) ; let Some (ref coverage_cx) = cx . coverage_cx else { return } ; let mut covfun_records = coverage_cx . instances_used () . into_iter () . sorted_by_cached_key (| & instance | tcx . symbol_name (instance) . name) . filter_map (| instance | prepare_covfun_record (tcx , instance , true)) . collect :: < Vec < _ > > () ; if cx . codegen_unit . is_code_coverage_dead_code_cgu () { unused :: prepare_covfun_records_for_unused_functions (cx , & mut covfun_records) ; } if covfun_records . is_empty () { return ; } let global_file_table = GlobalFileTable :: build (tcx , covfun_records . iter () . flat_map (| c | c . all_source_files ())) ; for covfun in & covfun_records { covfun :: generate_covfun_record (cx , & global_file_table , covfun) } generate_covmap_record (cx , covmap_version , & global_file_table . filenames_buffer) ; }
};
}
