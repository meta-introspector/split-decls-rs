macro_rules! deps {
    () => {
        ConstEvalError!();
        HirDatabase!();
    };
}

macro_rules! const_eval_static_query {
    () => {
        deps!();
        pub (crate) fn const_eval_static_query < 'db > (db : & 'db dyn HirDatabase , def : StaticId ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { let interner = DbInterner :: new_with (db , None , None) ; let body = db . monomorphized_mir_body (def . into () , GenericArgs :: new_from_iter (interner , []) , db . trait_environment_for_body (def . into ()) ,) ? ; let c = interpret_mir (db , body , false , None) ? . 0 ? ; Ok (c) }
    };
}

const_eval_static_query!();