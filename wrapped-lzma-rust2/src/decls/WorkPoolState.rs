macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! WorkPoolState {
    () => {
        deps!();
        # [doc = " States for the work pool."] # [derive (Debug , Clone , Copy , PartialEq)] pub (crate) enum WorkPoolState { # [doc = " Actively accepting work and dispatching to threads."] Dispatching , # [doc = " No more work will be submitted, draining existing work."] Draining , # [doc = " All work completed."] Finished , # [doc = " An error occurred."] Error , }
    };
}

WorkPoolState!()