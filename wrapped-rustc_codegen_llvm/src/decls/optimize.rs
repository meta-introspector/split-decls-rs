macro_rules! deps {
    () => {
        WriteBytecode!();
        DiagnosticHandlers!();
        LlvmCodegenBackend!();
        ModuleLlvm!();
        OptStage!();
        ThinBuffer!();
        CodegenDiagnosticsStage!();
        AutodiffStage!();
    };
}

macro_rules! optimize {
    () => {
        deps!();
        pub (crate) fn optimize (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < ModuleLlvm > , config : & ModuleConfig ,) { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_optimize" , & * module . name) ; let llcx = & * module . module_llvm . llcx ; let _handlers = DiagnosticHandlers :: new (cgcx , dcx , llcx , module , CodegenDiagnosticsStage :: Opt) ; if config . emit_no_opt_bc { let out = cgcx . output_filenames . temp_path_ext_for_cgu ("no-opt.bc" , & module . name , cgcx . invocation_temp . as_deref () ,) ; write_bitcode_to_file (module , & out) } if let Some (opt_level) = config . opt_level { let opt_stage = match cgcx . lto { Lto :: Fat => llvm :: OptStage :: PreLinkFatLTO , Lto :: Thin | Lto :: ThinLocal => llvm :: OptStage :: PreLinkThinLTO , _ if cgcx . opts . cg . linker_plugin_lto . enabled () => llvm :: OptStage :: PreLinkThinLTO , _ => llvm :: OptStage :: PreLinkNoLTO , } ; let consider_ad = cfg ! (llvm_enzyme) && config . autodiff . contains (& config :: AutoDiff :: Enable) ; let autodiff_stage = if consider_ad { AutodiffStage :: PreAD } else { AutodiffStage :: PostAD } ; let mut thin_lto_buffer = if (module . kind == ModuleKind :: Regular && config . emit_obj == EmitObj :: ObjectCode (BitcodeSection :: Full)) || config . emit_thin_lto_summary { Some (null_mut ()) } else { None } ; unsafe { llvm_optimize (cgcx , dcx , module , thin_lto_buffer . as_mut () , config , opt_level , opt_stage , autodiff_stage ,) } ; if let Some (thin_lto_buffer) = thin_lto_buffer { let thin_lto_buffer = unsafe { ThinBuffer :: from_raw_ptr (thin_lto_buffer) } ; module . thin_lto_buffer = Some (thin_lto_buffer . data () . to_vec ()) ; let bc_summary_out = cgcx . output_filenames . temp_path_for_cgu (OutputType :: ThinLinkBitcode , & module . name , cgcx . invocation_temp . as_deref () ,) ; if config . emit_thin_lto_summary && let Some (thin_link_bitcode_filename) = bc_summary_out . file_name () { let summary_data = thin_lto_buffer . thin_link_data () ; cgcx . prof . artifact_size ("llvm_bitcode_summary" , thin_link_bitcode_filename . to_string_lossy () , summary_data . len () as u64 ,) ; let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_module_codegen_emit_bitcode_summary" , & * module . name ,) ; if let Err (err) = fs :: write (& bc_summary_out , summary_data) { dcx . emit_err (WriteBytecode { path : & bc_summary_out , err }) ; } } } } }
    };
}

optimize!()