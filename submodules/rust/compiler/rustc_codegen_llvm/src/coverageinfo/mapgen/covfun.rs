mkuse!{use std :: ffi :: CString ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_codegen_ssa :: traits :: { BaseTypeCodegenMethods as _ , ConstCodegenMethods } ;}
mkuse!{use rustc_middle :: mir :: coverage :: { BasicCoverageBlock , CovTerm , CoverageIdsInfo , Expression , FunctionCoverageInfo , Mapping , MappingKind , Op , } ;}
mkuse!{use rustc_middle :: ty :: { Instance , TyCtxt } ;}
mkuse!{use rustc_span :: { SourceFile , Span } ;}
mkuse!{use rustc_target :: spec :: HasTargetSpec ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: common :: CodegenCx ;}
mkuse!{use crate :: coverageinfo :: mapgen :: { GlobalFileTable , VirtualFileMapping , spans } ;}
mkuse!{use crate :: coverageinfo :: { ffi , llvm_cov } ;}
mkuse!{use crate :: llvm ;}
mkitem!{mkstruct!{# [doc = " Intermediate coverage metadata for a single function, used to help build"] # [doc = " the final record that will be embedded in the `__llvm_covfun` section."] # [derive (Debug)] pub (crate) struct CovfunRecord < 'tcx > { # [doc = " Not used directly, but helpful in debug messages."] _instance : Instance < 'tcx > , mangled_function_name : & 'tcx str , source_hash : u64 , is_used : bool , virtual_file_mapping : VirtualFileMapping , expressions : Vec < ffi :: CounterExpression > , regions : ffi :: Regions , }}}
mkitem!{mkimpl!{impl < 'tcx > CovfunRecord < 'tcx > { # [doc = " Iterator that yields all source files referred to by this function's"] # [doc = " coverage mappings. Used to build the global file table for the CGU."] pub (crate) fn all_source_files (& self) -> impl Iterator < Item = & SourceFile > { self . virtual_file_mapping . local_file_table . iter () . map (Arc :: as_ref) } }}}

macro_rules! prepare_covfun_record_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_covfun_record in module {}", module_path!());
    };
}

mkfn!{
    prepare_covfun_record_introspect!();
    pub (crate) fn prepare_covfun_record < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , is_used : bool ,) -> Option < CovfunRecord < 'tcx > > { let fn_cov_info = tcx . instance_mir (instance . def) . function_coverage_info . as_deref () ? ; let ids_info = tcx . coverage_ids_info (instance . def) ? ; let expressions = prepare_expressions (ids_info) ; let mut covfun = CovfunRecord { _instance : instance , mangled_function_name : tcx . symbol_name (instance) . name , source_hash : if is_used { fn_cov_info . function_source_hash } else { 0 } , is_used , virtual_file_mapping : VirtualFileMapping :: default () , expressions , regions : ffi :: Regions :: default () , } ; fill_region_tables (tcx , fn_cov_info , ids_info , & mut covfun) ; if covfun . regions . has_no_regions () { debug ! (? covfun , "function has no mappings to embed; skipping") ; return None ; } Some (covfun) }
}

macro_rules! prepare_expressions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare_expressions in module {}", module_path!());
    };
}

mkfn!{
    prepare_expressions_introspect!();
    # [doc = " Convert the function's coverage-counter expressions into a form suitable for FFI."] fn prepare_expressions (ids_info : & CoverageIdsInfo) -> Vec < ffi :: CounterExpression > { let counter_for_term = ffi :: Counter :: from_term ; ids_info . expressions . iter () . map (move | & Expression { lhs , op , rhs } | ffi :: CounterExpression { lhs : counter_for_term (lhs) , kind : match op { Op :: Add => ffi :: ExprKind :: Add , Op :: Subtract => ffi :: ExprKind :: Subtract , } , rhs : counter_for_term (rhs) , }) . collect :: < Vec < _ > > () }
}

macro_rules! fill_region_tables_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fill_region_tables in module {}", module_path!());
    };
}

mkfn!{
    fill_region_tables_introspect!();
    # [doc = " Populates the mapping region tables in the current function's covfun record."] fn fill_region_tables < 'tcx > (tcx : TyCtxt < 'tcx > , fn_cov_info : & 'tcx FunctionCoverageInfo , ids_info : & 'tcx CoverageIdsInfo , covfun : & mut CovfunRecord < 'tcx > ,) { let counter_for_bcb = | bcb : BasicCoverageBlock | -> ffi :: Counter { let term = if covfun . is_used { ids_info . term_for_bcb [bcb] . expect ("every BCB in a mapping was given a term") } else { CovTerm :: Zero } ; ffi :: Counter :: from_term (term) } ; let source_map = tcx . sess . source_map () ; let Some (first_span) = (try { fn_cov_info . mappings . first () ? . span }) else { debug_assert ! (false , "function has no mappings: {covfun:?}") ; return ; } ; let source_file = source_map . lookup_source_file (first_span . lo ()) ; let local_file_id = covfun . virtual_file_mapping . push_file (& source_file) ; let discard_all = tcx . sess . coverage_options () . discard_all_spans_in_codegen ; let make_coords = | span : Span | { if discard_all { None } else { spans :: make_coords (source_map , & source_file , span) } } ; let ffi :: Regions { code_regions , expansion_regions : _ , branch_regions , } = & mut covfun . regions ; for & Mapping { ref kind , span } in & fn_cov_info . mappings { let Some (coords) = make_coords (span) else { continue } ; let cov_span = coords . make_coverage_span (local_file_id) ; match * kind { MappingKind :: Code { bcb } => { code_regions . push (ffi :: CodeRegion { cov_span , counter : counter_for_bcb (bcb) }) ; } MappingKind :: Branch { true_bcb , false_bcb } => { branch_regions . push (ffi :: BranchRegion { cov_span , true_counter : counter_for_bcb (true_bcb) , false_counter : counter_for_bcb (false_bcb) , }) ; } } } }
}

macro_rules! generate_covfun_record_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_covfun_record in module {}", module_path!());
    };
}

mkfn!{
    generate_covfun_record_introspect!();
    # [doc = " Generates the contents of the covfun record for this function, which"] # [doc = " contains the function's coverage mapping data. The record is then stored"] # [doc = " as a global variable in the `__llvm_covfun` section."] pub (crate) fn generate_covfun_record < 'tcx > (cx : & mut CodegenCx < '_ , 'tcx > , global_file_table : & GlobalFileTable , covfun : & CovfunRecord < 'tcx > ,) { let & CovfunRecord { _instance , mangled_function_name , source_hash , is_used , ref virtual_file_mapping , ref expressions , ref regions , } = covfun ; let Some (local_file_table) = virtual_file_mapping . resolve_all (global_file_table) else { debug_assert ! (false , "all local files should be present in the global file table: \
                global_file_table = {global_file_table:?}, \
                virtual_file_mapping = {virtual_file_mapping:?}") ; return ; } ; let coverage_mapping_buffer = llvm_cov :: write_function_mappings_to_buffer (& local_file_table , expressions , regions) ; let func_name_hash = llvm_cov :: hash_bytes (mangled_function_name . as_bytes ()) ; let covfun_record = cx . const_struct (& [cx . const_u64 (func_name_hash) , cx . const_u32 (coverage_mapping_buffer . len () as u32) , cx . const_u64 (source_hash) , cx . const_u64 (global_file_table . filenames_hash) , cx . const_bytes (& coverage_mapping_buffer) ,] , true ,) ; let u = if is_used { "u" } else { "" } ; let covfun_var_name = CString :: new (format ! ("__covrec_{func_name_hash:X}{u}")) . unwrap () ; debug ! ("function record var name: {covfun_var_name:?}") ; let covfun_global = llvm :: add_global (cx . llmod , cx . val_ty (covfun_record) , & covfun_var_name) ; llvm :: set_initializer (covfun_global , covfun_record) ; llvm :: set_global_constant (covfun_global , true) ; llvm :: set_linkage (covfun_global , llvm :: Linkage :: LinkOnceODRLinkage) ; llvm :: set_visibility (covfun_global , llvm :: Visibility :: Hidden) ; llvm :: set_section (covfun_global , cx . covfun_section_name ()) ; llvm :: set_alignment (covfun_global , Align :: EIGHT) ; if cx . target_spec () . supports_comdat () { llvm :: set_comdat (cx . llmod , covfun_global , & covfun_var_name) ; } cx . add_used_global (covfun_global) ; }
}