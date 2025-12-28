macro_rules! start {
    () => {
        pub (crate) fn start (path : & Path) { if ! transition (OFF , PENDING) { panic ! ("profiler already started") ; } let path = CString :: new (path . display () . to_string ()) . unwrap () ; if unsafe { ProfilerStart (path . as_ptr ()) } == 0 { panic ! ("profiler failed to start") } assert ! (transition (PENDING , ON)) ; }
    };
}

start!();