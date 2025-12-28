macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
        Registry!();
        ThreadPoolBuildError!();
        WorkerThread!();
    };
}

macro_rules! default_global_registry {
    () => {
        deps!();
        fn default_global_registry () -> Result < Arc < Registry > , ThreadPoolBuildError > { let result = Registry :: new (ThreadPoolBuilder :: new ()) ; let unsupported = matches ! (& result , Err (e) if e . is_unsupported ()) ; if unsupported && WorkerThread :: current () . is_null () { let builder = ThreadPoolBuilder :: new () . num_threads (1) . use_current_thread () ; let fallback_result = Registry :: new (builder) ; if fallback_result . is_ok () { return fallback_result ; } } result }
    };
}

default_global_registry!();