macro_rules! deps {
    () => {
        DiagnosticHandlers!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'a > Drop for DiagnosticHandlers < 'a > { fn drop (& mut self) { unsafe { llvm :: LLVMRustContextSetDiagnosticHandler (self . llcx , self . old_handler) ; drop (Box :: from_raw (self . data)) ; } } }
    };
}

impl_125!()