macro_rules! deps {
    () => {
        AttributePlace!();
        Builder!();
        ModuleLlvm!();
        CodegenCx!();
    };
}

macro_rules! compile_codegen_unit {
    () => {
        deps!();
        pub (crate) fn compile_codegen_unit (tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < ModuleLlvm > , u64) { let start_time = Instant :: now () ; let dep_node = tcx . codegen_unit (cgu_name) . codegen_dep_node (tcx) ; let (module , _) = tcx . dep_graph . with_task (dep_node , tcx , cgu_name , module_codegen , Some (dep_graph :: hash_result) ,) ; let time_to_codegen = start_time . elapsed () ; let cost = time_to_codegen . as_nanos () as u64 ; fn module_codegen (tcx : TyCtxt < '_ > , cgu_name : Symbol) -> ModuleCodegen < ModuleLlvm > { let cgu = tcx . codegen_unit (cgu_name) ; let _prof_timer = tcx . prof . generic_activity_with_arg_recorder ("codegen_module" , | recorder | { recorder . record_arg (cgu_name . to_string ()) ; recorder . record_arg (cgu . size_estimate () . to_string ()) ; }) ; let llvm_module = ModuleLlvm :: new (tcx , cgu_name . as_str ()) ; { let mut cx = CodegenCx :: new (tcx , cgu , & llvm_module) ; let mono_items = cx . codegen_unit . items_in_deterministic_order (cx . tcx) ; for & (mono_item , data) in & mono_items { mono_item . predefine :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , data . linkage , data . visibility ,) ; } for & (mono_item , item_data) in & mono_items { mono_item . define :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , item_data) ; } if let Some (entry) = maybe_create_entry_wrapper :: < Builder < '_ , '_ , '_ > > (& cx , cx . codegen_unit) { let attrs = attributes :: sanitize_attrs (& cx , SanitizerSet :: empty ()) ; attributes :: apply_to_llfn (entry , llvm :: AttributePlace :: Function , & attrs) ; } if cx . sess () . instrument_coverage () { cx . coverageinfo_finalize () ; } if ! cx . used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.used" , & cx . used_statics) ; } if ! cx . compiler_used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.compiler.used" , & cx . compiler_used_statics) ; } for & (old_g , new_g) in cx . statics_to_rauw () . borrow () . iter () { unsafe { llvm :: LLVMReplaceAllUsesWith (old_g , new_g) ; llvm :: LLVMDeleteGlobal (old_g) ; } } if cx . sess () . opts . debuginfo != DebugInfo :: None { cx . debuginfo_finalize () ; } } ModuleCodegen :: new_regular (cgu_name . to_string () , llvm_module) } (module , cost) }
    };
}

compile_codegen_unit!()