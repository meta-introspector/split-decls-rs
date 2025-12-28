macro_rules! deps {
    () => {
        CachedModuleCodegen!();
        ExtraBackendMethods!();
        WorkItem!();
        Message!();
        Coordinator!();
    };
}

macro_rules! submit_post_lto_module_to_llvm {
    () => {
        deps!();
        pub (crate) fn submit_post_lto_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : CachedModuleCodegen ,) { let llvm_work_item = WorkItem :: CopyPostLtoArtifacts (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost : 0 })) ; }
    };
}

submit_post_lto_module_to_llvm!();