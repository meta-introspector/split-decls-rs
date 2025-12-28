macro_rules! deps {
    () => {
        ThreadLocalMode!();
    };
}

macro_rules! to_llvm_tls_model {
    () => {
        deps!();
        fn to_llvm_tls_model (tls_model : TlsModel) -> llvm :: ThreadLocalMode { match tls_model { TlsModel :: GeneralDynamic => llvm :: ThreadLocalMode :: GeneralDynamic , TlsModel :: LocalDynamic => llvm :: ThreadLocalMode :: LocalDynamic , TlsModel :: InitialExec => llvm :: ThreadLocalMode :: InitialExec , TlsModel :: LocalExec => llvm :: ThreadLocalMode :: LocalExec , TlsModel :: Emulated => llvm :: ThreadLocalMode :: GeneralDynamic , } }
    };
}

to_llvm_tls_model!();