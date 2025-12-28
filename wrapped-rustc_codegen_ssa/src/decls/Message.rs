macro_rules! deps {
    () => {
        WorkItem!();
        SerializedModule!();
        WorkerFatalError!();
        WorkItemResult!();
        WriteBackendMethods!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " Messages sent to the coordinator."] pub (crate) enum Message < B : WriteBackendMethods > { # [doc = " A jobserver token has become available. Sent from the jobserver helper"] # [doc = " thread."] Token (io :: Result < Acquired >) , # [doc = " The backend has finished processing a work item for a codegen unit."] # [doc = " Sent from a backend worker thread."] WorkItem { result : Result < WorkItemResult < B > , Option < WorkerFatalError > > } , # [doc = " The frontend has finished generating something (backend IR or a"] # [doc = " post-LTO artifact) for a codegen unit, and it should be passed to the"] # [doc = " backend. Sent from the main thread."] CodegenDone { llvm_work_item : WorkItem < B > , cost : u64 } , # [doc = " Similar to `CodegenDone`, but for reusing a pre-LTO artifact"] # [doc = " Sent from the main thread."] AddImportOnlyModule { module_data : SerializedModule < B :: ModuleBuffer > , work_product : WorkProduct , } , # [doc = " The frontend has finished generating everything for all codegen units."] # [doc = " Sent from the main thread."] CodegenComplete , # [doc = " Some normal-ish compiler error occurred, and codegen should be wound"] # [doc = " down. Sent from the main thread."] CodegenAborted , }
    };
}

Message!();