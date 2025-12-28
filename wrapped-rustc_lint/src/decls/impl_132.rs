macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'tcx > ty :: layout :: HasTypingEnv < 'tcx > for LateContext < 'tcx > { # [inline] fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . typing_env () } }
    };
}

impl_132!();