macro_rules! DiagnosticHandlerTy {
    () => {
        pub (crate) type DiagnosticHandlerTy = unsafe extern "C" fn (& DiagnosticInfo , * mut c_void) ;
    };
}

DiagnosticHandlerTy!()