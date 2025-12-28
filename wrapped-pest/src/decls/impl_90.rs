macro_rules! deps {
    () => {
        CallLimitTracker!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Default for CallLimitTracker { fn default () -> Self { let limit = CALL_LIMIT . load (Ordering :: Relaxed) ; let current_call_limit = if limit > 0 { Some ((0 , limit)) } else { None } ; Self { current_call_limit } } }
    };
}

impl_90!()