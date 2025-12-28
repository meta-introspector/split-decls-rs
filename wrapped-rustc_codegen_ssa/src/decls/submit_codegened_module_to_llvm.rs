macro_rules! deps {
    () => {
        Coordinator!();
        ExtraBackendMethods!();
        WorkItem!();
        Message!();
        ModuleCodegen!();
    };
}

macro_rules! submit_codegened_module_to_llvm {
    () => {
        deps!();
        pub (crate) fn submit_codegened_module_to_llvm < B : ExtraBackendMethods > (coordinator : & Coordinator < B > , module : ModuleCodegen < B :: Module > , cost : u64 ,) { let llvm_work_item = WorkItem :: Optimize (module) ; drop (coordinator . sender . send (Message :: CodegenDone :: < B > { llvm_work_item , cost })) ; }
    };
}

submit_codegened_module_to_llvm!();