/* FP:queries.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0001
/* FP:queries.rs-0002 */ use std :: any :: Any ;
/* FP:queries.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0002
/* FP:queries.rs-0004 */ use std :: sync :: Arc ;
/* FP:queries.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0003
/* FP:queries.rs-0006 */ use crate :: rustc_codegen_ssa :: CodegenResults ;
/* FP:queries.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0004
/* FP:queries.rs-0008 */ use crate :: rustc_codegen_ssa :: traits :: CodegenBackend ;
/* FP:queries.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0005
/* FP:queries.rs-0010 */ use crate :: rustc_data_structures :: svh :: Svh ;
/* FP:queries.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0006
/* FP:queries.rs-0012 */ use crate :: rustc_complete :: timings :: TimingSection ;
/* FP:queries.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0007
/* FP:queries.rs-0014 */ use crate :: rustc_complete :: def_id :: LOCAL_CRATE ;
/* FP:queries.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0008
/* FP:queries.rs-0016 */ use crate :: rustc_metadata :: EncodedMetadata ;
/* FP:queries.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0009
/* FP:queries.rs-0018 */ use crate :: rustc_complete :: dep_graph :: DepGraph ;
/* FP:queries.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0010
/* FP:queries.rs-0020 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:queries.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0011
/* FP:queries.rs-0022 */ use crate :: rustc_complete :: Session ;
/* FP:queries.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0012
/* FP:queries.rs-0024 */ use crate :: rustc_complete :: config :: { self , OutputFilenames , OutputType } ;
/* FP:queries.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0013
/* FP:queries.rs-0026 */ use crate :: errors :: FailedWritingFile ;
/* FP:queries.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_USE_0014
/* FP:queries.rs-0028 */ use crate :: passes ;
/* FP:queries.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_STRUCT_0015
/* FP:queries.rs-0030 */ pub struct Linker { dep_graph : DepGraph , output_filenames : Arc < OutputFilenames > , crate_hash : Option < Svh > , metadata : EncodedMetadata , ongoing_codegen : Box < dyn Any > , }
/* FP:queries.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_queries_IMPL_0016
/* FP:queries.rs-0032 */ impl Linker { pub fn codegen_and_build_linker (tcx : TyCtxt < '_ > , codegen_backend : & dyn CodegenBackend ,) -> Linker { let (ongoing_codegen , metadata) = passes :: start_codegen (codegen_backend , tcx) ; Linker { dep_graph : tcx . dep_graph . clone () , output_filenames : Arc :: clone (tcx . output_filenames (())) , crate_hash : if tcx . needs_crate_hash () { Some (tcx . crate_hash (LOCAL_CRATE)) } else { None } , metadata , ongoing_codegen , } } pub fn link (self , sess : & Session , codegen_backend : & dyn CodegenBackend) { let (codegen_results , mut work_products) = sess . time ("finish_ongoing_codegen" , | | { codegen_backend . join_codegen (self . ongoing_codegen , sess , & self . output_filenames) }) ; sess . timings . end_section (sess . dcx () , TimingSection :: Codegen) ; if sess . opts . incremental . is_some () && let Some (path) = self . metadata . path () && let Some ((id , product)) = rustc_incremental :: copy_cgu_workproduct_to_incr_comp_cache_dir (sess , "metadata" , & [("rmeta" , path)] , & [] ,) { work_products . insert (id , product) ; } sess . dcx () . abort_if_errors () ; let _timer = sess . timer ("link") ; sess . time ("serialize_work_products" , | | { rustc_incremental :: save_work_product_index (sess , & self . dep_graph , work_products) }) ; let prof = sess . prof . clone () ; prof . generic_activity ("drop_dep_graph") . run (move | | drop (self . dep_graph)) ; rustc_incremental :: finalize_session_directory (sess , self . crate_hash) ; if ! sess . opts . output_types . keys () . any (| & i | i == OutputType :: Exe || i == OutputType :: Metadata) { return ; } if sess . opts . unstable_opts . no_link { let rlink_file = self . output_filenames . with_extension (config :: RLINK_EXT) ; CodegenResults :: serialize_rlink (sess , & rlink_file , & codegen_results , & self . metadata , & * self . output_filenames ,) . unwrap_or_else (| error | { sess . dcx () . emit_fatal (FailedWritingFile { path : & rlink_file , error }) }) ; return ; } let _timer = sess . prof . verbose_generic_activity ("link_crate") ; let _timing = sess . timings . section_guard (sess . dcx () , TimingSection :: Linking) ; codegen_backend . link (sess , codegen_results , self . metadata , & self . output_filenames) } }