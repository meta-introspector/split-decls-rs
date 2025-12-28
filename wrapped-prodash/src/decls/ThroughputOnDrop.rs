macro_rules! deps {
    () => {
        NestedProgress!();
    };
}

macro_rules! ThroughputOnDrop {
    () => {
        deps!();
        # [doc = " Emit a message with throughput information when the instance is dropped."] pub struct ThroughputOnDrop < T : NestedProgress > (T , Instant) ;
    };
}

ThroughputOnDrop!();