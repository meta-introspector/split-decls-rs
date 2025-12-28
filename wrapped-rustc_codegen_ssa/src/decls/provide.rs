macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { crate :: back :: symbol_export :: provide (providers) ; crate :: base :: provide (providers) ; crate :: target_features :: provide (providers) ; crate :: codegen_attrs :: provide (providers) ; providers . queries . global_backend_features = | _tcx : TyCtxt < '_ > , () | vec ! [] ; }
    };
}

provide!();