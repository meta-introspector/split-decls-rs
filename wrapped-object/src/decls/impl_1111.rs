macro_rules! deps {
    () => {
        SegmentId!();
        Id!();
    };
}

macro_rules! impl_1111 {
    () => {
        deps!();
        impl Id for SegmentId { fn index (& self) -> usize { self . 0 } }
    };
}

impl_1111!()