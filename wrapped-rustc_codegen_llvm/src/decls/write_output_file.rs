macro_rules! deps {
    () => {
        FileType!();
        LlvmError!();
        LLVMRustResult!();
    };
}

macro_rules! write_output_file {
    () => {
        deps!();
        fn write_output_file < 'll > (dcx : DiagCtxtHandle < '_ > , target : & 'll llvm :: TargetMachine , no_builtins : bool , m : & 'll llvm :: Module , output : & Path , dwo_output : Option < & Path > , file_type : llvm :: FileType , self_profiler_ref : & SelfProfilerRef , verify_llvm_ir : bool ,) { debug ! ("write_output_file output={:?} dwo_output={:?}" , output , dwo_output) ; let output_c = path_to_c_string (output) ; let dwo_output_c ; let dwo_output_ptr = if let Some (dwo_output) = dwo_output { dwo_output_c = path_to_c_string (dwo_output) ; dwo_output_c . as_ptr () } else { std :: ptr :: null () } ; let result = unsafe { let pm = llvm :: LLVMCreatePassManager () ; llvm :: LLVMAddAnalysisPasses (target , pm) ; llvm :: LLVMRustAddLibraryInfo (pm , m , no_builtins) ; llvm :: LLVMRustWriteOutputFile (target , pm , m , output_c . as_ptr () , dwo_output_ptr , file_type , verify_llvm_ir ,) } ; if result == llvm :: LLVMRustResult :: Success { let artifact_kind = match file_type { llvm :: FileType :: ObjectFile => "object_file" , llvm :: FileType :: AssemblyFile => "assembly_file" , } ; record_artifact_size (self_profiler_ref , artifact_kind , output) ; if let Some (dwo_file) = dwo_output { record_artifact_size (self_profiler_ref , "dwo_file" , dwo_file) ; } } result . into_result () . unwrap_or_else (| () | llvm_err (dcx , LlvmError :: WriteOutput { path : output })) }
    };
}

write_output_file!()