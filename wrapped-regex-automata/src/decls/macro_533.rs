macro_rules! deps {
    () => {
        Counters!();
        PikeVM!();
    };
}

macro_rules! macro_533 {
    () => {
        deps!();
        # [cfg (feature = "internal-instrument-pikevm")] std :: thread_local ! { # [doc = " Effectively global state used to keep track of instrumentation"] # [doc = " counters. The \"proper\" way to do this is to thread it through the"] # [doc = " PikeVM, but it makes the code quite icky. Since this is just a"] # [doc = " debugging feature, we're content to relegate it to thread local"] # [doc = " state. When instrumentation is enabled, the counters are reset at the"] # [doc = " beginning of every search and printed (with the 'trace' log level) at"] # [doc = " the end of every search."] static COUNTERS : RefCell < Counters > = RefCell :: new (Counters :: empty ()) ; }
    };
}

macro_533!();