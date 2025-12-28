macro_rules! CallLimitTracker {
    () => {
        # [derive (Debug)] struct CallLimitTracker { current_call_limit : Option < (usize , usize) > , }
    };
}

CallLimitTracker!()