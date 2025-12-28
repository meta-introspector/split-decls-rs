macro_rules! deps {
    () => {
        Parameter!();
        ParameterCollector!();
    };
}

macro_rules! parameters_for {
    () => {
        deps!();
        # [doc = " If `include_nonconstraining` is false, returns the list of parameters that are"] # [doc = " constrained by `value` - i.e., the value of each parameter in the list is"] # [doc = " uniquely determined by `value` (see RFC 447). If it is true, return the list"] # [doc = " of parameters whose values are needed in order to constrain `value` - these"] # [doc = " differ, with the latter being a superset, in the presence of projections."] pub (crate) fn parameters_for < 'tcx > (tcx : TyCtxt < 'tcx > , value : impl TypeFoldable < TyCtxt < 'tcx > > , include_nonconstraining : bool ,) -> Vec < Parameter > { let mut collector = ParameterCollector { parameters : vec ! [] , include_nonconstraining } ; let value = if ! include_nonconstraining { tcx . expand_free_alias_tys (value) } else { value } ; value . visit_with (& mut collector) ; collector . parameters }
    };
}

parameters_for!();