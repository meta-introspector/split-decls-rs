macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! LlvmError {
    () => {
        deps!();
        # [derive (Diagnostic)] pub enum LlvmError < 'a > { # [diag (codegen_llvm_write_output)] WriteOutput { path : & 'a Path } , # [diag (codegen_llvm_target_machine)] CreateTargetMachine { triple : SmallCStr } , # [diag (codegen_llvm_run_passes)] RunLlvmPasses , # [diag (codegen_llvm_serialize_module)] SerializeModule { name : & 'a str } , # [diag (codegen_llvm_write_ir)] WriteIr { path : & 'a Path } , # [diag (codegen_llvm_prepare_thin_lto_context)] PrepareThinLtoContext , # [diag (codegen_llvm_load_bitcode)] LoadBitcode { name : CString } , # [diag (codegen_llvm_write_thinlto_key)] WriteThinLtoKey { err : std :: io :: Error } , # [diag (codegen_llvm_prepare_thin_lto_module)] PrepareThinLtoModule , # [diag (codegen_llvm_parse_bitcode)] ParseBitcode , # [diag (codegen_llvm_prepare_autodiff)] PrepareAutoDiff { src : String , target : String , error : String } , }
    };
}

LlvmError!()