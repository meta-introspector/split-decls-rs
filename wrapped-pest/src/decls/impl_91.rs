macro_rules! deps {
    () => {
        CallLimitTracker!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl CallLimitTracker { fn limit_reached (& self) -> bool { self . current_call_limit . is_some_and (| (current , limit) | current >= limit) } fn increment_depth (& mut self) { if let Some ((current , _)) = & mut self . current_call_limit { * current += 1 ; } } }
    };
}

impl_91!()