macro_rules! extend_exported_symbols {
    () => {
        # [doc = " On amdhsa, `gpu-kernel` functions have an associated metadata object with a `.kd` suffix."] # [doc = " Add it to the symbols list for all kernel functions, so that it is exported in the linked"] # [doc = " object."] pub (crate) fn extend_exported_symbols < 'tcx > (symbols : & mut Vec < (String , SymbolExportKind) > , tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , instantiating_crate : CrateNum ,) { let (callconv , _) = calling_convention_for_symbol (tcx , symbol) ; if callconv != CanonAbi :: GpuKernel || tcx . sess . target . os != "amdhsa" { return ; } let undecorated = symbol_name_for_instance_in_crate (tcx , symbol , instantiating_crate) ; symbols . push ((format ! ("{undecorated}.kd") , SymbolExportKind :: Data)) ; }
    };
}

extend_exported_symbols!();