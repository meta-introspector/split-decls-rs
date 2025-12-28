macro_rules! deps {
    () => {
        SegmentId!();
    };
}

macro_rules! impl_1112 {
    () => {
        deps!();
        impl IdPrivate for SegmentId { fn new (id : usize) -> Self { SegmentId (id) } }
    };
}

impl_1112!()