macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
        DiagnosticHandlers!();
        ModuleLlvm!();
        CodegenDiagnosticsStage!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > DiagnosticHandlers < 'a > { pub (crate) fn new (cgcx : & 'a CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < 'a > , llcx : & 'a llvm :: Context , module : & ModuleCodegen < ModuleLlvm > , stage : CodegenDiagnosticsStage ,) -> Self { let remark_passes_all : bool ; let remark_passes : Vec < CString > ; match & cgcx . remark { Passes :: All => { remark_passes_all = true ; remark_passes = Vec :: new () ; } Passes :: Some (passes) => { remark_passes_all = false ; remark_passes = passes . iter () . map (| name | CString :: new (name . as_str ()) . unwrap ()) . collect () ; } } ; let remark_passes : Vec < * const c_char > = remark_passes . iter () . map (| name : & CString | name . as_ptr ()) . collect () ; let remark_file = cgcx . remark_dir . as_ref () . map (| dir | { let stage_suffix = match stage { CodegenDiagnosticsStage :: Codegen => "codegen" , CodegenDiagnosticsStage :: Opt => "opt" , CodegenDiagnosticsStage :: LTO => "lto" , } ; dir . join (format ! ("{}.{stage_suffix}.opt.yaml" , module . name)) }) . and_then (| dir | dir . to_str () . and_then (| p | CString :: new (p) . ok ())) ; let pgo_available = cgcx . opts . cg . profile_use . is_some () ; let data = Box :: into_raw (Box :: new ((cgcx , dcx))) ; unsafe { let old_handler = llvm :: LLVMRustContextGetDiagnosticHandler (llcx) ; llvm :: LLVMRustContextConfigureDiagnosticHandler (llcx , diagnostic_handler , data . cast () , remark_passes_all , remark_passes . as_ptr () , remark_passes . len () , remark_file . as_ref () . map (| dir | dir . as_ptr ()) . unwrap_or (std :: ptr :: null ()) , pgo_available ,) ; DiagnosticHandlers { data , llcx , old_handler } } } }
    };
}

impl_124!();