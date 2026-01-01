/* FP:mapgen.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0001
/* FP:mapgen.rs-0002 */ use std :: assert_matches :: assert_matches ;
/* FP:mapgen.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0002
/* FP:mapgen.rs-0004 */ use std :: sync :: Arc ;
/* FP:mapgen.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0003
/* FP:mapgen.rs-0006 */ use itertools :: Itertools ;
/* FP:mapgen.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0004
/* FP:mapgen.rs-0008 */ use crate :: rustc_abi :: Align ;
/* FP:mapgen.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0005
/* FP:mapgen.rs-0010 */ use crate :: rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods , ConstCodegenMethods } ;
/* FP:mapgen.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0006
/* FP:mapgen.rs-0012 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:mapgen.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0007
/* FP:mapgen.rs-0014 */ use crate :: rustc_index :: IndexVec ;
/* FP:mapgen.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0008
/* FP:mapgen.rs-0016 */ use rustc_macros :: TryFromU32 ;
/* FP:mapgen.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0009
/* FP:mapgen.rs-0018 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:mapgen.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0010
/* FP:mapgen.rs-0020 */ use crate :: rustc_complete :: RemapFileNameExt ;
/* FP:mapgen.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0011
/* FP:mapgen.rs-0022 */ use crate :: rustc_complete :: config :: RemapPathScopeComponents ;
/* FP:mapgen.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0012
/* FP:mapgen.rs-0024 */ use crate :: rustc_complete :: { SourceFile , StableSourceFileId } ;
/* FP:mapgen.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0013
/* FP:mapgen.rs-0026 */ use tracing :: debug ;
/* FP:mapgen.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0014
/* FP:mapgen.rs-0028 */ use crate :: common :: CodegenCx ;
/* FP:mapgen.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0015
/* FP:mapgen.rs-0030 */ use crate :: coverageinfo :: llvm_cov ;
/* FP:mapgen.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0016
/* FP:mapgen.rs-0032 */ use crate :: coverageinfo :: mapgen :: covfun :: prepare_covfun_record ;
/* FP:mapgen.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_USE_0017
/* FP:mapgen.rs-0034 */ use crate :: llvm ;
/* FP:mapgen.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_MOD_0018
/* FP:mapgen.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_MOD_0019
/* FP:mapgen.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_MOD_0020
/* FP:mapgen.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_ENUM_0021
/* FP:mapgen.rs-0042 */ # [doc = " Version number that will be included the `__llvm_covmap` section header."] # [doc = " Corresponds to LLVM's `llvm::coverage::CovMapVersion` (in `CoverageMapping.h`),"] # [doc = " or at least the subset that we know and care about."] # [doc = ""] # [doc = " Note that version `n` is encoded as `(n-1)`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord , TryFromU32)] enum CovmapVersion { # [doc = " Used by LLVM 18 onwards."] Version7 = 6 , }
/* FP:mapgen.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_IMPL_0022
/* FP:mapgen.rs-0044 */ impl CovmapVersion { fn to_u32 (self) -> u32 { self as u32 } }
/* FP:mapgen.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_FN_0023
/* FP:mapgen.rs-0046 */ # [doc = " Generates and exports the coverage map, which is embedded in special"] # [doc = " linker sections in the final binary."] # [doc = ""] # [doc = " Those sections are then read and understood by LLVM's `llvm-cov` tool,"] # [doc = " which is distributed in the `llvm-tools` rustup component."] pub (crate) fn finalize (cx : & mut CodegenCx < '_ , '_ >) { let tcx = cx . tcx ; let covmap_version = CovmapVersion :: try_from (llvm_cov :: mapping_version ()) . unwrap_or_else (| raw_version : u32 | { panic ! ("unknown coverage mapping version reported by `llvm-wrapper`: {raw_version}") }) ; assert_matches ! (covmap_version , CovmapVersion :: Version7) ; debug ! ("Generating coverage map for CodegenUnit: `{}`" , cx . codegen_unit . name ()) ; let Some (ref coverage_cx) = cx . coverage_cx else { return } ; let mut covfun_records = coverage_cx . instances_used () . into_iter () . sorted_by_cached_key (| & instance | tcx . symbol_name (instance) . name) . filter_map (| instance | prepare_covfun_record (tcx , instance , true)) . collect :: < Vec < _ > > () ; if cx . codegen_unit . is_code_coverage_dead_code_cgu () { unused :: prepare_covfun_records_for_unused_functions (cx , & mut covfun_records) ; } if covfun_records . is_empty () { return ; } let global_file_table = GlobalFileTable :: build (tcx , covfun_records . iter () . flat_map (| c | c . all_source_files ())) ; for covfun in & covfun_records { covfun :: generate_covfun_record (cx , & global_file_table , covfun) } generate_covmap_record (cx , covmap_version , & global_file_table . filenames_buffer) ; }
/* FP:mapgen.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_STRUCT_0024
/* FP:mapgen.rs-0048 */ # [doc = " Maps \"global\" (per-CGU) file ID numbers to their underlying source file paths."] # [derive (Debug)] struct GlobalFileTable { # [doc = " This \"raw\" table doesn't include the working dir, so a file's"] # [doc = " global ID is its index in this set **plus one**."] raw_file_table : FxIndexMap < StableSourceFileId , String > , # [doc = " The file table in encoded form (possibly compressed), which can be"] # [doc = " included directly in this CGU's `__llvm_covmap` record."] filenames_buffer : Vec < u8 > , # [doc = " Truncated hash of the bytes in `filenames_buffer`."] # [doc = ""] # [doc = " The `llvm-cov` tool uses this hash to associate each covfun record with"] # [doc = " its corresponding filenames table, since the final binary will typically"] # [doc = " contain multiple covmap records from different compilation units."] filenames_hash : u64 , }
/* FP:mapgen.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_IMPL_0025
/* FP:mapgen.rs-0050 */ impl GlobalFileTable { # [doc = " Builds a \"global file table\" for this CGU, mapping numeric IDs to"] # [doc = " path strings."] fn build < 'a > (tcx : TyCtxt < '_ > , all_files : impl Iterator < Item = & 'a SourceFile >) -> Self { let mut raw_file_table = FxIndexMap :: default () ; for file in all_files { raw_file_table . entry (file . stable_id) . or_insert_with (| | { file . name . for_scope (tcx . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () . into_owned () }) ; } let mut table = Vec :: with_capacity (raw_file_table . len () + 1) ; let base_dir = tcx . sess . opts . working_dir . for_scope (tcx . sess , RemapPathScopeComponents :: MACRO) . to_string_lossy () ; table . push (base_dir . as_ref ()) ; table . extend (raw_file_table . values () . map (| name | name . as_str ())) ; let filenames_buffer = llvm_cov :: write_filenames_to_buffer (& table) ; let filenames_hash = llvm_cov :: hash_bytes (& filenames_buffer) ; Self { raw_file_table , filenames_buffer , filenames_hash } } fn get_existing_id (& self , file : & SourceFile) -> Option < GlobalFileId > { let raw_id = self . raw_file_table . get_index_of (& file . stable_id) ? ; Some (GlobalFileId :: from_usize (raw_id + 1)) } }
/* FP:mapgen.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_MACRO_0026
/* FP:mapgen.rs-0052 */ crate :: rustc_index :: newtype_index ! { # [doc = " An index into the CGU's overall list of file paths. The underlying paths"] # [doc = " will be embedded in the `__llvm_covmap` linker section."] struct GlobalFileId { } }
/* FP:mapgen.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_MACRO_0027
/* FP:mapgen.rs-0054 */ crate :: rustc_index :: newtype_index ! { # [doc = " An index into a function's list of global file IDs. That underlying list"] # [doc = " of local-to-global mappings will be embedded in the function's record in"] # [doc = " the `__llvm_covfun` linker section."] struct LocalFileId { } }
/* FP:mapgen.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_STRUCT_0028
/* FP:mapgen.rs-0056 */ # [doc = " Holds a mapping from \"local\" (per-function) file IDs to their corresponding"] # [doc = " source files."] # [derive (Debug , Default)] struct VirtualFileMapping { local_file_table : IndexVec < LocalFileId , Arc < SourceFile > > , }
/* FP:mapgen.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_IMPL_0029
/* FP:mapgen.rs-0058 */ impl VirtualFileMapping { fn push_file (& mut self , source_file : & Arc < SourceFile >) -> LocalFileId { self . local_file_table . push (Arc :: clone (source_file)) } # [doc = " Resolves all of the filenames in this local file mapping to a list of"] # [doc = " global file IDs in its CGU, for inclusion in this function's"] # [doc = " `__llvm_covfun` record."] # [doc = ""] # [doc = " The global file IDs are returned as `u32` to make FFI easier."] fn resolve_all (& self , global_file_table : & GlobalFileTable) -> Option < Vec < u32 > > { self . local_file_table . iter () . map (| file | try { let id = global_file_table . get_existing_id (file) ? ; GlobalFileId :: as_u32 (id) }) . collect :: < Option < Vec < _ > > > () } }
/* FP:mapgen.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_coverageinfo_mapgen_FN_0030