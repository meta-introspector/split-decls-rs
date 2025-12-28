macro_rules! push_generic_params {
    () => {
        pub fn push_generic_params < 'tcx > (tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx > , output : & mut String ,) { let _prof = tcx . prof . generic_activity ("compute_debuginfo_type_name") ; let mut visited = FxHashSet :: default () ; push_generic_params_internal (tcx , args , output , & mut visited) ; }
    };
}

push_generic_params!();