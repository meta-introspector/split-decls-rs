macro_rules! deps {
    () => {
        TraitEnvironment!();
        ConstEvalError!();
        HirDatabase!();
    };
}

macro_rules! const_eval_cycle_result {
    () => {
        deps!();
        pub (crate) fn const_eval_cycle_result < 'db > (_db : & 'db dyn HirDatabase , _salsa_id : salsa :: Id , _const_id : ConstId , _subst : GenericArgs < 'db > , _trait_env : Option < Arc < TraitEnvironment < 'db > > > ,) -> Result < Const < 'db > , ConstEvalError < 'db > > { Err (ConstEvalError :: MirLowerError (MirLowerError :: Loop)) }
    };
}

const_eval_cycle_result!();