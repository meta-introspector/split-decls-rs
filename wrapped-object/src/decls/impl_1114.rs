macro_rules! deps {
    () => {
        Id!();
        Item!();
        SegmentId!();
        Segment!();
    };
}

macro_rules! impl_1114 {
    () => {
        deps!();
        impl < 'data > Item for Segment < 'data > { type Id = SegmentId ; fn is_deleted (& self) -> bool { self . delete } }
    };
}

impl_1114!();