macro_rules! deps {
    () => {
        JobRef!();
    };
}

macro_rules! JobFifo {
    () => {
        deps!();
        # [doc = " Indirect queue to provide FIFO job priority."] pub (super) struct JobFifo { inner : Injector < JobRef > , }
    };
}

JobFifo!()