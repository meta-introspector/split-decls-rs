macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < 'tcx , 'll > HasTypingEnv < 'tcx > for CodegenCx < 'll , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }
    };
}

impl_229!();