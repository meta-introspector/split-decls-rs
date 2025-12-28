macro_rules! deps {
    () => {
        OptimizationDiagnostic!();
        Linker!();
        InlineAsmDiagnostic!();
    };
}

macro_rules! Diagnostic {
    () => {
        deps!();
        pub (crate) enum Diagnostic < 'll > { Optimization (OptimizationDiagnostic < 'll >) , InlineAsm (InlineAsmDiagnostic) , PGO (& 'll DiagnosticInfo) , Linker (& 'll DiagnosticInfo) , Unsupported (& 'll DiagnosticInfo) , # [doc = " LLVM has other types that we do not wrap here."] # [expect (dead_code)] UnknownDiagnostic (& 'll DiagnosticInfo) , }
    };
}

Diagnostic!();