macro_rules! deps {
    () => {
        CanAccessMutGlobal!();
    };
}

macro_rules! turn_into_const_value {
    () => {
        deps!();
        # [instrument (skip (tcx) , level = "debug" , ret)] pub (crate) fn turn_into_const_value < 'tcx > (tcx : TyCtxt < 'tcx > , constant : ConstAlloc < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , GlobalId < 'tcx > > ,) -> ConstValue { let cid = key . value ; let def_id = cid . instance . def . def_id () ; let is_static = tcx . is_static (def_id) ; let ecx = mk_eval_cx_to_read_const_val (tcx , tcx . def_span (key . value . instance . def_id ()) , key . typing_env , CanAccessMutGlobal :: from (is_static) ,) ; let mplace = ecx . raw_const_to_mplace (constant) . expect ("can only fail if layout computation failed, \
        which should have given a good error before ever invoking this function" ,) ; assert ! (! is_static || cid . promoted . is_some () , "the `eval_to_const_value_raw` query should not be used for statics, use `eval_to_allocation` instead") ; op_to_const (& ecx , & mplace . into () , false) }
    };
}

turn_into_const_value!()