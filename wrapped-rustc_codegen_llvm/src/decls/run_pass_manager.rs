macro_rules! deps {
    () => {
        SimpleCx!();
        LlvmCodegenBackend!();
        OptStage!();
        ModuleLlvm!();
        AutodiffStage!();
    };
}

macro_rules! run_pass_manager {
    () => {
        deps!();
        pub (crate) fn run_pass_manager (cgcx : & CodegenContext < LlvmCodegenBackend > , dcx : DiagCtxtHandle < '_ > , module : & mut ModuleCodegen < ModuleLlvm > , thin : bool ,) { let _timer = cgcx . prof . generic_activity_with_arg ("LLVM_lto_optimize" , & * module . name) ; let config = & cgcx . module_config ; debug ! ("running the pass manager") ; let opt_stage = if thin { llvm :: OptStage :: ThinLTO } else { llvm :: OptStage :: FatLTO } ; let opt_level = config . opt_level . unwrap_or (config :: OptLevel :: No) ; let enable_ad = config . autodiff . contains (& config :: AutoDiff :: Enable) ; let enable_gpu = config . offload . contains (& config :: Offload :: Enable) ; let stage = if thin { write :: AutodiffStage :: PreAD } else { if enable_ad { write :: AutodiffStage :: DuringAD } else { write :: AutodiffStage :: PostAD } } ; if enable_ad { enable_autodiff_settings (& config . autodiff) ; } unsafe { write :: llvm_optimize (cgcx , dcx , module , None , config , opt_level , opt_stage , stage) ; } if enable_gpu && ! thin { let cx = SimpleCx :: new (module . module_llvm . llmod () , & module . module_llvm . llcx , cgcx . pointer_size) ; crate :: builder :: gpu_offload :: handle_gpu_code (cgcx , & cx) ; } if cfg ! (llvm_enzyme) && enable_ad && ! thin { let opt_stage = llvm :: OptStage :: FatLTO ; let stage = write :: AutodiffStage :: PostAD ; if ! config . autodiff . contains (& config :: AutoDiff :: NoPostopt) { unsafe { write :: llvm_optimize (cgcx , dcx , module , None , config , opt_level , opt_stage , stage) ; } } if config . autodiff . contains (& config :: AutoDiff :: PrintModFinal) { unsafe { llvm :: LLVMDumpModule (module . module_llvm . llmod ()) } ; } } debug ! ("lto done") ; }
    };
}

run_pass_manager!();