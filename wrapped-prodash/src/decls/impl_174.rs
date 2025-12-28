macro_rules! deps {
    () => {
        ThroughputOnDrop!();
        NestedProgress!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < T : NestedProgress > Drop for ThroughputOnDrop < T > { fn drop (& mut self) { self . 0 . show_throughput (self . 1) } }
    };
}

impl_174!()