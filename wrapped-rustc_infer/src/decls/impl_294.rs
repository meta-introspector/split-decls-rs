macro_rules! deps {
    () => {
        ProjectionCacheKey!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < 'tcx > ProjectionCacheKey < 'tcx > { pub fn new (term : ty :: AliasTerm < 'tcx > , param_env : ty :: ParamEnv < 'tcx >) -> Self { Self { term , param_env } } }
    };
}

impl_294!()