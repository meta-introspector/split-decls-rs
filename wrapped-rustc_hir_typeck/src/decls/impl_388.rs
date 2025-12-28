macro_rules! deps {
    () => {
        FnCtxt!();
        EagerlyNormalizeConsts!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < 'tcx > EagerlyNormalizeConsts < 'tcx > { fn new (fcx : & FnCtxt < '_ , 'tcx >) -> Self { EagerlyNormalizeConsts { tcx : fcx . tcx , typing_env : fcx . typing_env (fcx . param_env) } } }
    };
}

impl_388!();