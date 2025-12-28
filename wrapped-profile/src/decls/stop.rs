macro_rules! stop {
    () => {
        pub (crate) fn stop () { if ! transition (ON , PENDING) { panic ! ("profiler is not started") } unsafe { ProfilerStop () } ; assert ! (transition (PENDING , OFF)) ; }
    };
}

stop!()