macro_rules! deps {
    () => {
        HirDatabase!();
        ConstEvalError!();
    };
}

macro_rules! const_eval_static_cycle_result {
    () => {
        deps!();
        pub (crate) fn const_eval_static_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _salsa_id : salsa :: Id , _static_id : StaticId ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { Err (ConstEvalError :: MirLowerError (MirLowerError :: Loop)) }
    };
}

const_eval_static_cycle_result!()