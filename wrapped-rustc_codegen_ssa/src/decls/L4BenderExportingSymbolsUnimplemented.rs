macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! L4BenderExportingSymbolsUnimplemented {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_L4Bender_exporting_symbols_unimplemented)] pub (crate) struct L4BenderExportingSymbolsUnimplemented ;
    };
}

L4BenderExportingSymbolsUnimplemented!()