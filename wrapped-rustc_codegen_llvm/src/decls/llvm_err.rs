macro_rules! deps {
    () => {
        LlvmError!();
        WithLlvmError!();
    };
}

macro_rules! llvm_err {
    () => {
        deps!();
        pub (crate) fn llvm_err < 'a > (dcx : DiagCtxtHandle < '_ > , err : LlvmError < 'a >) -> ! { match llvm :: last_error () { Some (llvm_err) => dcx . emit_fatal (WithLlvmError (err , llvm_err)) , None => dcx . emit_fatal (err) , } }
    };
}

llvm_err!()