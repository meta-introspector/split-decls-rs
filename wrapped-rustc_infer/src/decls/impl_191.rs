macro_rules! deps {
    () => {
        SnapshotVarData!();
        VariableLengths!();
        InferCtxt!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl SnapshotVarData { fn new (infcx : & InferCtxt < '_ > , vars_pre_snapshot : VariableLengths) -> SnapshotVarData { let mut inner = infcx . inner . borrow_mut () ; let region_vars = inner . unwrap_region_constraints () . vars_since_snapshot (vars_pre_snapshot . region_constraints_len) ; let type_vars = inner . type_variables () . vars_since_snapshot (vars_pre_snapshot . type_var_len) ; let int_vars = vars_since_snapshot (& inner . int_unification_table () , vars_pre_snapshot . int_var_len) ; let float_vars = vars_since_snapshot (& inner . float_unification_table () , vars_pre_snapshot . float_var_len) ; let const_vars = const_vars_since_snapshot (& mut inner . const_unification_table () , vars_pre_snapshot . const_var_len ,) ; SnapshotVarData { region_vars , type_vars , int_vars , float_vars , const_vars } } fn is_empty (& self) -> bool { let SnapshotVarData { region_vars , type_vars , int_vars , float_vars , const_vars } = self ; region_vars . 0 . is_empty () && type_vars . 0 . is_empty () && int_vars . is_empty () && float_vars . is_empty () && const_vars . 0 . is_empty () } }
    };
}

impl_191!()