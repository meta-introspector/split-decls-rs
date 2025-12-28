macro_rules! deps {
    () => {
        MatchError!();
    };
}

macro_rules! gave_up {
    () => {
        deps!();
        # [doc = " A convenience routine for constructing a \"gave up\" match error."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn gave_up (offset : usize) -> MatchError { MatchError :: gave_up (offset) }
    };
}

gave_up!()