macro_rules! deps {
    () => {
        DiagnosticHandlers!();
        ModuleLlvm!();
        LlvmCodegenBackend!();
        CodegenDiagnosticsStage!();
        Linker!();
        LlvmError!();
        ModuleBuffer!();
    };
}

macro_rules! fat_lto {
    () => {
        deps!();
        fn fat_lto (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , modules : Vec < FatLtoInput < LlvmCodegenBackend > > , mut serialized_modules : Vec < (SerializedModule < ModuleBuffer > , CString) > , symbols_below_threshold : & [* const libc :: c_char] ,) -> ModuleCodegen < ModuleLlvm > { let _timer = cgcx . prof . generic_activity ("LLVM_fat_lto_build_monolithic_module") ; info ! ("going for a fat lto") ; let mut in_memory = Vec :: new () ; for module in modules { match module { FatLtoInput :: InMemory (m) => in_memory . push (m) , FatLtoInput :: Serialized { name , buffer } => { info ! ("pushing serialized module {:?}" , name) ; serialized_modules . push ((buffer , CString :: new (name) . unwrap ())) ; } } } let costliest_module = in_memory . iter () . enumerate () . filter (| & (_ , module) | module . kind == ModuleKind :: Regular) . map (| (i , module) | { let cost = unsafe { llvm :: LLVMRustModuleCost (module . module_llvm . llmod ()) } ; (cost , i) }) . max () ; let module : ModuleCodegen < ModuleLlvm > = match costliest_module { Some ((_cost , i)) => in_memory . remove (i) , None => { assert ! (! serialized_modules . is_empty () , "must have at least one serialized module") ; let (buffer , name) = serialized_modules . remove (0) ; info ! ("no in-memory regular modules to choose from, parsing {:?}" , name) ; let llvm_module = ModuleLlvm :: parse (cgcx , & name , buffer . data () , dcx) ; ModuleCodegen :: new_regular (name . into_string () . unwrap () , llvm_module) } } ; { let (llcx , llmod) = { let llvm = & module . module_llvm ; (& llvm . llcx , llvm . llmod ()) } ; info ! ("using {:?} as a base module" , module . name) ; let _handler = DiagnosticHandlers :: new (cgcx , dcx , llcx , & module , CodegenDiagnosticsStage :: LTO) ; for module in in_memory { let buffer = ModuleBuffer :: new (module . module_llvm . llmod ()) ; let llmod_id = CString :: new (& module . name [..]) . unwrap () ; serialized_modules . push ((SerializedModule :: Local (buffer) , llmod_id)) ; } serialized_modules . sort_by (| module1 , module2 | module1 . 1 . cmp (& module2 . 1)) ; let mut linker = Linker :: new (llmod) ; for (bc_decoded , name) in serialized_modules { let _timer = cgcx . prof . generic_activity_with_arg_recorder ("LLVM_fat_lto_link_module" , | recorder | { recorder . record_arg (format ! ("{name:?}")) }) ; info ! ("linking {:?}" , name) ; let data = bc_decoded . data () ; linker . add (data) . unwrap_or_else (| () | write :: llvm_err (dcx , LlvmError :: LoadBitcode { name })) ; } drop (linker) ; save_temp_bitcode (cgcx , & module , "lto.input") ; unsafe { let ptr = symbols_below_threshold . as_ptr () ; llvm :: LLVMRustRunRestrictionPass (llmod , ptr as * const * const libc :: c_char , symbols_below_threshold . len () as libc :: size_t ,) ; } save_temp_bitcode (cgcx , & module , "lto.after-restriction") ; } module }
    };
}

fat_lto!()