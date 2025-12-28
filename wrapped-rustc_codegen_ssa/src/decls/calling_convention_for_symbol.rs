macro_rules! calling_convention_for_symbol {
    () => {
        fn calling_convention_for_symbol < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > ,) -> (CanonAbi , & 'tcx [rustc_target :: callconv :: ArgAbi < 'tcx , Ty < 'tcx > >]) { let instance = match symbol { ExportedSymbol :: NonGeneric (def_id) | ExportedSymbol :: Generic (def_id , _) if tcx . is_static (def_id) => { None } ExportedSymbol :: NonGeneric (def_id) => Some (Instance :: mono (tcx , def_id)) , ExportedSymbol :: Generic (def_id , args) => Some (Instance :: new_raw (def_id , args)) , ExportedSymbol :: DropGlue (..) => None , ExportedSymbol :: AsyncDropGlueCtorShim (..) => None , ExportedSymbol :: AsyncDropGlue (..) => None , ExportedSymbol :: NoDefId (..) => None , ExportedSymbol :: ThreadLocalShim (..) => None , } ; instance . map (| i | { tcx . fn_abi_of_instance (ty :: TypingEnv :: fully_monomorphized () . as_query_input ((i , ty :: List :: empty ())) ,) . unwrap_or_else (| _ | bug ! ("fn_abi_of_instance({i:?}) failed")) }) . map (| fnabi | (fnabi . conv , & fnabi . args [..])) . unwrap_or ((CanonAbi :: Rust , & [])) }
    };
}

calling_convention_for_symbol!();