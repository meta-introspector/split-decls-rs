macro_rules! deps {
    () => {
        ExtraBackendMethods!();
        CopyPathBuf!();
        CompiledModule!();
        ModuleKind!();
        CachedModuleCodegen!();
        NoSavedObjectFile!();
        EmitObj!();
        WorkItemResult!();
        CodegenContext!();
    };
}

macro_rules! execute_copy_from_cache_work_item {
    () => {
        deps!();
        fn execute_copy_from_cache_work_item < B : ExtraBackendMethods > (cgcx : & CodegenContext < B > , module : CachedModuleCodegen ,) -> WorkItemResult < B > { let _timer = cgcx . prof . generic_activity_with_arg ("codegen_copy_artifacts_from_incr_cache" , & * module . name) ; let incr_comp_session_dir = cgcx . incr_comp_session_dir . as_ref () . unwrap () ; let mut links_from_incr_cache = Vec :: new () ; let mut load_from_incr_comp_dir = | output_path : PathBuf , saved_path : & str | { let source_file = in_incr_comp_dir (incr_comp_session_dir , saved_path) ; debug ! ("copying preexisting module `{}` from {:?} to {}" , module . name , source_file , output_path . display ()) ; match link_or_copy (& source_file , & output_path) { Ok (_) => { links_from_incr_cache . push (source_file) ; Some (output_path) } Err (error) => { cgcx . create_dcx () . handle () . emit_err (errors :: CopyPathBuf { source_file , output_path , error , }) ; None } } } ; let dwarf_object = module . source . saved_files . get ("dwo") . as_ref () . and_then (| saved_dwarf_object_file | { let dwarf_obj_out = cgcx . output_filenames . split_dwarf_path (cgcx . split_debuginfo , cgcx . split_dwarf_kind , & module . name , cgcx . invocation_temp . as_deref () ,) . expect ("saved dwarf object in work product but `split_dwarf_path` returned `None`" ,) ; load_from_incr_comp_dir (dwarf_obj_out , saved_dwarf_object_file) }) ; let mut load_from_incr_cache = | perform , output_type : OutputType | { if perform { let saved_file = module . source . saved_files . get (output_type . extension ()) ? ; let output_path = cgcx . output_filenames . temp_path_for_cgu (output_type , & module . name , cgcx . invocation_temp . as_deref () ,) ; load_from_incr_comp_dir (output_path , & saved_file) } else { None } } ; let module_config = & cgcx . module_config ; let should_emit_obj = module_config . emit_obj != EmitObj :: None ; let assembly = load_from_incr_cache (module_config . emit_asm , OutputType :: Assembly) ; let llvm_ir = load_from_incr_cache (module_config . emit_ir , OutputType :: LlvmAssembly) ; let bytecode = load_from_incr_cache (module_config . emit_bc , OutputType :: Bitcode) ; let object = load_from_incr_cache (should_emit_obj , OutputType :: Object) ; if should_emit_obj && object . is_none () { cgcx . create_dcx () . handle () . emit_fatal (errors :: NoSavedObjectFile { cgu_name : & module . name }) } WorkItemResult :: Finished (CompiledModule { links_from_incr_cache , kind : ModuleKind :: Regular , name : module . name , object , dwarf_object , bytecode , assembly , llvm_ir , }) }
    };
}

execute_copy_from_cache_work_item!()