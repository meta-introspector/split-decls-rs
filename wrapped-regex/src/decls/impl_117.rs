macro_rules! deps {
    () => {
        CaptureNames!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < 'r > core :: iter :: FusedIterator for CaptureNames < 'r > { }
    };
}

impl_117!();