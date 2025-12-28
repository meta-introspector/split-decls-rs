macro_rules! ProjectionCacheKey {
    () => {
        # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] pub struct ProjectionCacheKey < 'tcx > { term : ty :: AliasTerm < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , }
    };
}

ProjectionCacheKey!();