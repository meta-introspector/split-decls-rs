macro_rules! deps {
    () => {
        Probe!();
    };
}

macro_rules! test_probe_layout {
    () => {
        deps!();
        # [test] fn test_probe_layout () { use std :: alloc :: Layout ; use std :: mem ; let probe = Probe :: new () ; assert_eq ! (Layout :: new ::< sys :: io_uring_probe > () . size () + mem :: size_of ::< sys :: io_uring_probe_op > () * 256 , Layout :: for_value (& probe . 0) . size ()) ; assert_eq ! (Layout :: new ::< sys :: io_uring_probe > () . align () , Layout :: for_value (& probe . 0) . align ()) ; }
    };
}

test_probe_layout!()