macro_rules! deps {
    () => {
        ProbeAndOps!();
    };
}

macro_rules! Probe {
    () => {
        deps!();
        # [doc = " Information about what `io_uring` features the kernel supports."] # [doc = ""] # [doc = " You can fill this in with [`register_probe`](crate::Submitter::register_probe)."] pub struct Probe (ProbeAndOps) ;
    };
}

Probe!();