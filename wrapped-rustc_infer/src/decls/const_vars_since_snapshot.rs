macro_rules! deps {
    () => {
        UnificationTable!();
        ConstVariableOrigin!();
        ConstVariableValue!();
        ConstVidKey!();
    };
}

macro_rules! const_vars_since_snapshot {
    () => {
        deps!();
        fn const_vars_since_snapshot < 'tcx > (table : & mut UnificationTable < '_ , 'tcx , ConstVidKey < 'tcx > > , snapshot_var_len : usize ,) -> (Range < ConstVid > , Vec < ConstVariableOrigin >) { let range = vars_since_snapshot (table , snapshot_var_len) ; let range = range . start . vid .. range . end . vid ; (range . clone () , range . map (| index | match table . probe_value (index) { ConstVariableValue :: Known { value : _ } => { ConstVariableOrigin { param_def_id : None , span : rustc_span :: DUMMY_SP } } ConstVariableValue :: Unknown { origin , universe : _ } => origin , }) . collect () ,) }
    };
}

const_vars_since_snapshot!();