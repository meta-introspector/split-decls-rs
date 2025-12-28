macro_rules! record_llvm_cgu_instructions_stats {
    () => {
        fn record_llvm_cgu_instructions_stats (prof : & SelfProfilerRef , llmod : & llvm :: Module) { if ! prof . enabled () { return ; } let raw_stats = llvm :: build_string (| s | unsafe { llvm :: LLVMRustModuleInstructionStats (llmod , s) }) . expect ("cannot get module instruction stats") ; # [derive (serde :: Deserialize)] struct InstructionsStats { module : String , total : u64 , } let InstructionsStats { module , total } = serde_json :: from_str (& raw_stats) . expect ("cannot parse llvm cgu instructions stats") ; prof . artifact_size ("cgu_instructions" , module , total) ; }
    };
}

record_llvm_cgu_instructions_stats!();