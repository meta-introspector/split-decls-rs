macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! const_param_ty_query {
    () => {
        deps!();
        pub (crate) fn const_param_ty_query < 'db > (db : & 'db dyn HirDatabase , def : ConstParamId) -> Ty < 'db > { db . const_param_ty_with_diagnostics (def) . 0 }
    };
}

const_param_ty_query!();