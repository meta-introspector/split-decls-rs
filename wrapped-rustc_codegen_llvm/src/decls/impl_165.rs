macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'tcx > ty :: layout :: HasTypingEnv < 'tcx > for Builder < '_ , '_ , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . cx . typing_env () } }
    };
}

impl_165!()