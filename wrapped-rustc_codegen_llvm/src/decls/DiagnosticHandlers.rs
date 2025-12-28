macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
    };
}

macro_rules! DiagnosticHandlers {
    () => {
        deps!();
        pub (crate) struct DiagnosticHandlers < 'a > { data : * mut (& 'a CodegenContext < LlvmCodegenBackend > , DiagCtxtHandle < 'a >) , llcx : & 'a llvm :: Context , old_handler : Option < & 'a llvm :: DiagnosticHandler > , }
    };
}

DiagnosticHandlers!();