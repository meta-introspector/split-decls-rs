macro_rules! deps {
    () => {
        AllocError!();
        TryReserveError!();
    };
}

macro_rules! handle_reserve {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] # [inline (always)] fn handle_reserve (result : Result < () , TryReserveError >) { match result . map_err (| e | e . kind ()) { Err (CapacityOverflow) => capacity_overflow () , Err (AllocError { layout , .. }) => handle_alloc_error (layout) , Ok (()) => { } } }
    };
}

handle_reserve!();