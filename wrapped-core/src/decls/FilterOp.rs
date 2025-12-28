macro_rules! FilterOp {
    () => {
        # [doc = " Operation that `unpark_filter` should perform for each thread."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum FilterOp { # [doc = " Unpark the thread and continue scanning the list of parked threads."] Unpark , # [doc = " Don't unpark the thread and continue scanning the list of parked threads."] Skip , # [doc = " Don't unpark the thread and stop scanning the list of parked threads."] Stop , }
    };
}

FilterOp!()