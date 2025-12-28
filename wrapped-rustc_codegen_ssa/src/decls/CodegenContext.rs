macro_rules! deps {
    () => {
        ModuleConfig!();
        WriteBackendMethods!();
        TargetMachineFactoryFn!();
        SharedEmitter!();
    };
}

macro_rules! CodegenContext {
    () => {
        deps!();
        # [doc = " Additional resources used by optimize_and_codegen (not module specific)"] # [derive (Clone)] pub struct CodegenContext < B : WriteBackendMethods > { pub prof : SelfProfilerRef , pub lto : Lto , pub save_temps : bool , pub fewer_names : bool , pub time_trace : bool , pub opts : Arc < config :: Options > , pub crate_types : Vec < CrateType > , pub output_filenames : Arc < OutputFilenames > , pub invocation_temp : Option < String > , pub module_config : Arc < ModuleConfig > , pub allocator_config : Arc < ModuleConfig > , pub tm_factory : TargetMachineFactoryFn < B > , pub msvc_imps_needed : bool , pub is_pe_coff : bool , pub target_can_use_split_dwarf : bool , pub target_arch : String , pub target_is_like_darwin : bool , pub target_is_like_aix : bool , pub split_debuginfo : rustc_target :: spec :: SplitDebuginfo , pub split_dwarf_kind : rustc_session :: config :: SplitDwarfKind , pub pointer_size : Size , # [doc = " All commandline args used to invoke the compiler, with @file args fully expanded."] # [doc = " This will only be used within debug info, e.g. in the pdb file on windows"] # [doc = " This is mainly useful for other tools that reads that debuginfo to figure out"] # [doc = " how to call the compiler with the same arguments."] pub expanded_args : Vec < String > , # [doc = " Emitter to use for diagnostics produced during codegen."] pub diag_emitter : SharedEmitter , # [doc = " LLVM optimizations for which we want to print remarks."] pub remark : Passes , # [doc = " Directory into which should the LLVM optimization remarks be written."] # [doc = " If `None`, they will be written to stderr."] pub remark_dir : Option < PathBuf > , # [doc = " The incremental compilation session directory, or None if we are not"] # [doc = " compiling incrementally"] pub incr_comp_session_dir : Option < PathBuf > , # [doc = " `true` if the codegen should be run in parallel."] # [doc = ""] # [doc = " Depends on [`ExtraBackendMethods::supports_parallel()`] and `-Zno_parallel_backend`."] pub parallel : bool , }
    };
}

CodegenContext!()