macro_rules! deps {
    () => {
        ParamIndexRemapper!();
    };
}

macro_rules! build_generic_args {
    () => {
        deps!();
        fn build_generic_args < 'tcx > (tcx : TyCtxt < 'tcx > , sig_id : DefId , def_id : LocalDefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: GenericArgsRef < 'tcx > { let caller_generics = tcx . generics_of (def_id) ; let callee_generics = tcx . generics_of (sig_id) ; let mut remap_table = FxHashMap :: default () ; for caller_param in & caller_generics . own_params { let callee_index = callee_generics . param_def_id_to_index (tcx , caller_param . def_id) . unwrap () ; remap_table . insert (callee_index , caller_param . index) ; } let mut folder = ParamIndexRemapper { tcx , remap_table } ; args . fold_with (& mut folder) }
    };
}

build_generic_args!();