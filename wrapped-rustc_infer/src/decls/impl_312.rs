macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < 'tcx , T : Copy > Obligation < 'tcx , T > { pub fn as_goal (& self) -> solve :: Goal < 'tcx , T > { solve :: Goal { param_env : self . param_env , predicate : self . predicate } } }
    };
}

impl_312!();