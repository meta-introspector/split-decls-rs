macro_rules! exporting_symbol_name_for_instance_in_crate {
    () => {
        pub (crate) fn exporting_symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , cnum : CrateNum ,) -> String { let undecorated = symbol_name_for_instance_in_crate (tcx , symbol , cnum) ; maybe_emutls_symbol_name (tcx , symbol , & undecorated) . unwrap_or (undecorated) }
    };
}

exporting_symbol_name_for_instance_in_crate!()