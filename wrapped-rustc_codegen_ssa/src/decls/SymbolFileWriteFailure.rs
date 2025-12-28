macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! SymbolFileWriteFailure {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_symbol_file_write_failure)] pub (crate) struct SymbolFileWriteFailure { pub error : Error , }
    };
}

SymbolFileWriteFailure!()