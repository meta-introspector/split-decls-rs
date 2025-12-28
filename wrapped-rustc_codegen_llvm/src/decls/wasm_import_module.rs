macro_rules! wasm_import_module {
    () => {
        fn wasm_import_module (tcx : TyCtxt < '_ > , id : DefId) -> Option < & String > { tcx . wasm_import_module_map (id . krate) . get (& id) }
    };
}

wasm_import_module!()