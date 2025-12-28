macro_rules! deps {
    () => {
        Machine!();
        InterpCx!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'tcx , M > layout :: HasTypingEnv < 'tcx > for InterpCx < 'tcx , M > where M : Machine < 'tcx > , { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env } }
    };
}

impl_209!()