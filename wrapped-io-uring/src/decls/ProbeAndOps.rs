macro_rules! deps {
    () => {
        Probe!();
    };
}

macro_rules! ProbeAndOps {
    () => {
        deps!();
        # [repr (C)] struct ProbeAndOps (sys :: io_uring_probe , [sys :: io_uring_probe_op ; Probe :: COUNT]) ;
    };
}

ProbeAndOps!()