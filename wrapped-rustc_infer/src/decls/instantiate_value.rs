macro_rules! deps {
    () => {
        CanonicalInstantiator!();
    };
}

macro_rules! instantiate_value {
    () => {
        deps!();
        # [doc = " Instantiate the values from `var_values` into `value`. `var_values`"] # [doc = " must be values for the set of canonical variables that appear in"] # [doc = " `value`."] pub (super) fn instantiate_value < 'tcx , T > (tcx : TyCtxt < 'tcx > , var_values : & CanonicalVarValues < 'tcx > , value : T ,) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { if var_values . var_values . is_empty () { return value ; } value . fold_with (& mut CanonicalInstantiator { tcx , current_index : ty :: INNERMOST , var_values : var_values . var_values , cache : Default :: default () , }) }
    };
}

instantiate_value!();